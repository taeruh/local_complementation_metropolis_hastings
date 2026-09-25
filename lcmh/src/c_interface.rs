use std::mem::ManuallyDrop;

use crate::search::{self, CoolingConfiguration, SearchArtifacts};

// On most platforms (appartly, all that are currently supported by Rust) C's size_t is
// equivalent to Rust's usize
// TODO: get the actual size_t via a build script...
#[allow(non_camel_case_types)]
type size_t = usize;

mod graph_transformation {
    use std::ops::BitOrAssign;

    use super::size_t;
    use crate::graph::Graph;

    const CHUNK_SIZE: usize = 64;

    /// A helper struct to access that graph data structure as they are represented in
    /// Cabaliser.
    pub struct CabaliserGraph {
        n_qubits: size_t,
        // do not wrap (the outer) pointer into a &ref because this requires quite a few
        // guarantees that I don't want to enforce here for simplicity.
        slices: *mut *mut u64,
    }

    impl CabaliserGraph {
        /// Creates a new CabaliserGraph, cf.
        /// [lcmh_transform_cabaliser_graph_to_lcmh_graph](super::lcmh_transform_cabaliser_graph_to_lcmh_graph).
        ///
        /// # Safety
        /// `slices` must be initialised for `n_qubits` and each slice `slices[i]`
        /// must again be valid for `n_qubits`.
        pub unsafe fn new(n_qubits: size_t, slices: *mut *mut u64) -> Self {
            CabaliserGraph { n_qubits, slices }
        }

        /// Builds a [Graph] from the given [CabaliserGraph].
        pub fn to_graph(c_graph: CabaliserGraph) -> Graph {
            if c_graph.n_qubits == 0 {
                return Graph::new(0);
            }

            let num_chunks = c_graph.n_qubits.div_ceil(CHUNK_SIZE);
            // note (for below) that this index points to the last chunk of all the
            // rows which is initialised as [Self::new] guarantees that
            let last_chunk_idx = num_chunks - 1;
            let max_bit_in_last_chunk = (c_graph.n_qubits - 1) % CHUNK_SIZE;
            let mut graph = Graph::new(c_graph.n_qubits);

            let mut potentially_add_edge =
                |chunk: u64, chunk_idx: usize, i: usize, j: usize| {
                    if (chunk & (1 << j)) != 0 {
                        graph.add_edge(i, chunk_idx * CHUNK_SIZE + j);
                    }
                };

            for i in 0..(c_graph.n_qubits - 1) {
                // Safety: we have i < n_qubits - 1 <= n_qubits - 1
                let slice = unsafe { *c_graph.slices.add(i) };
                let start_j = i + 1;
                let current_chunk_idx = start_j / CHUNK_SIZE;
                if current_chunk_idx != num_chunks - 1 {
                    // Safety: we have current_chunk_idx = (i + 1) / CHUNK_SIZE <=
                    // (n_qubits - 1) / CHUNK_SIZE = num_chunks - 1 = last_chunk_idx,
                    // which is a valid index
                    let chunk = unsafe { *slice.add(current_chunk_idx) };
                    for j in start_j..CHUNK_SIZE {
                        potentially_add_edge(chunk, current_chunk_idx, i, j);
                    }
                }
                for chunk_idx in (current_chunk_idx + 1)..(num_chunks - 1) {
                    // Safety: we have chunk_idx < num_chunks - 1 = last_chunk_idx, which
                    // is a valid index
                    let chunk = unsafe { *slice.add(chunk_idx) };
                    for j in 0..CHUNK_SIZE {
                        potentially_add_edge(chunk, chunk_idx, i, j);
                    }
                }
                // Safety: last_chunk_idx is a valid index
                let chunk = unsafe { *slice.add(last_chunk_idx) };
                for j in (start_j % CHUNK_SIZE)..(max_bit_in_last_chunk + 1) {
                    potentially_add_edge(chunk, last_chunk_idx, i, j);
                }
            }

            graph
        }

        /// Writes the given [Graph] into the given `slices_buffer` that is assumed to be
        /// initalised with zeros.
        ///
        /// # Safety
        /// `slices_buffer` must be valid for `graph.num_nodes()` and each slice
        /// `slices_buffer[i]` must again be valid for `graph.num_nodes()`.
        pub unsafe fn from_graph(
            graph: &Graph,
            slices_buffer: *mut *mut u64,
        ) -> CabaliserGraph {
            let c_graph = CabaliserGraph {
                n_qubits: graph.num_nodes(),
                slices: slices_buffer,
            };
            for i in 0..graph.num_nodes() {
                // Safety: we have i < graph.num_nodes() <= graph.num_nodes - 1, which is
                // a valid index
                let slice = unsafe { *slices_buffer.add(i) };
                for &neighbour in graph.get_neighbours(i).unwrap() {
                    // Safety: we have neighbour / CHUNK_SIZE < graph.num_nodes() /
                    // CHUNK_SIZE <= num_chunks - 1 = last_chunk_idx, which is a valid
                    // index
                    let chunk = unsafe { &mut *slice.add(neighbour / CHUNK_SIZE) };
                    chunk.bitor_assign(1 << (neighbour % CHUNK_SIZE))
                }
            }
            c_graph
        }
    }
}

use graph_transformation::CabaliserGraph;

/// The graph encoding we use for the LCMH search.
pub type LcmhGraph = crate::graph::Graph;

/// Transforms a Cabaliser graph into an LCMH graph.
///
/// Creates a new LCMH from the given number of qubits, `n_qubits` and
/// the backing memory of the graph that is accessed via the `slices`. It is
/// assumed that the graph is stored as a binary adjacency matrix, where each row
/// can be accessed via the `slices` pointer, and each row-pointer then points to
/// the 64-bit chunks that encode the row.
///
/// # Arguments
/// - `n_qubits`: The number of qubits/nodes in the graph.
/// - `slices`: A pointer to the backing memory of the graph
///
/// # Returns
/// - a Box/pointer to the newly created LCMH graph.
///
/// # Safety
/// `slices` must be initialised for `n_qubits` and each slice `slices[i]`
/// must again be valid for `n_qubits`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_transform_cabaliser_graph_to_lcmh_graph(
    n_qubits: size_t,
    slices: *mut *mut u64,
) -> Box<LcmhGraph> {
    let c_graph = unsafe { CabaliserGraph::new(n_qubits, slices) };
    Box::new(CabaliserGraph::to_graph(c_graph))
}

/// Transforms an LCMH graph into a Cabaliser graph.
///
/// Creates a new Cabaliser graph from the given LCMH graph and writing into a backing
/// memory accessed via the `slices_buffer`, assuming that the buffer is initialised with
/// zeros; cf. [lcmh_transform_cabaliser_graph_to_lcmh_graph] for the encoding.
///
/// # Arguments
/// - `graph`: The LCMH graph to be transformed.
/// - `slices_buffer`: A pointer to the buffer memory where the Cabaliser graph will be
///   written to.
///
/// # Returns
/// - the number of nodes/qubits in the graph.
///
/// # Safety
/// `slices_buffer` must be valid for `graph.num_nodes()` and each slice
/// `slices_buffer[i]` must again be valid for `graph.num_nodes()`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_transform_lcmh_graph_to_cabaliser_graph(
    graph: &LcmhGraph,
    // must be valid for `graph.num_nodes()` and initialised to zeros
    slices_buffer: *mut *mut u64,
) -> size_t {
    unsafe { CabaliserGraph::from_graph(graph, slices_buffer) };
    graph.num_nodes()
}

/// Clones the given LCMH graph.
///
/// # Arguments
/// - `graph`: The LCMH graph to be cloned.
///
/// # Returns
/// - a Box/pointer to the newly created LCMH graph.
#[unsafe(no_mangle)]
// pub extern "C" fn lcmh_clone_lcmh_graph(graph: &LcmhGraph) -> Box<LcmhGraph> {
pub extern "C" fn lcmh_clone_lcmh_graph(graph: &LcmhGraph) -> Box<LcmhGraph> {
    Box::new(graph.clone())
}

/// Frees the given LCMH graph.
///
/// # Arguments
/// - `graph`: The LCMH graph to be freed.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_free_lcmh_graph(graph: Box<LcmhGraph>) {
    drop(graph);
}

/// Get the number of nodes in the given LCMH graph.
///
/// # Arguments
/// - `graph`: The LCMH graph.
///
/// # Returns
/// - the number of nodes in the graph.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_get_num_edges(graph: &LcmhGraph) -> size_t {
    graph.num_edges()
}

/// Get the maximum degree of the given LCMH graph.
///
/// # Arguments
/// - `graph`: The LCMH graph.
///
/// # Returns
/// - the maximum degree of the graph.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_get_max_degree(graph: &LcmhGraph) -> size_t {
    graph.max_degree()
}

/// Get the number of neighbours of the given node in the given LCMH graph.
///
/// # Arguments
/// - `graph`: The LCMH graph.
/// - `node`: The node for which to get the number of neighbours.
///
/// # Returns
/// - the number of neighbours of the node.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_get_nodes_num_neighbours(
    graph: &LcmhGraph,
    node: size_t,
) -> size_t {
    graph.get_neighbours(node).expect("Node not in graph").len()
}

/// The cooling configuration for the search.
///
/// # Invariants
///
/// `num_betas` must be equal to the length of `betas` and `num_steps_per_beta`.
#[repr(C)]
pub struct LcmhCoolingConfiguration {
    /// The number of betas.
    pub num_betas: usize,
    /// The betas.
    pub betas: *mut f64,
    /// The number of "steps" per beta. Note that each "step" consists of
    /// `graph.num_nodes()` local complementation draws (not necessarily accepted).
    pub num_steps_per_beta: *mut usize,
}

impl From<LcmhCoolingConfiguration> for CoolingConfiguration {
    fn from(c: LcmhCoolingConfiguration) -> Self {
        let mut betas = Vec::with_capacity(c.num_betas);
        let mut num_steps_per_beta = Vec::with_capacity(c.num_betas);
        for i in 0..c.num_betas {
            // Safety: we have i < c.num_betas <= c.num_betas, which is a valid index
            // according to the invariants of [LcmhCoolingConfiguration].
            unsafe {
                betas.push(*c.betas.add(i));
                num_steps_per_beta.push(*c.num_steps_per_beta.add(i));
            }
        }
        CoolingConfiguration { betas, num_steps_per_beta }
    }
}

/// Encoding for the local Clifford operations according to the encoding in Cabaliser
pub type LcmhSingleQubitClifford = u8;

#[repr(C)]
#[derive(Debug)]
pub struct LcmhSingleQubitCliffordOperation {
    /// The operation that is applied.
    pub operation: LcmhSingleQubitClifford,
    /// The node on which the operation is applied.
    pub node: usize,
}

/// Collection of search artifacts (apart from the transformed graph) that are returned by
/// the search functions.
///
/// Use [lcmh_free_search_artifacts] to free the memory allocated by the individual
/// artifact pointers (do not free them manually).
#[repr(C)]
pub struct LcmhSearchArtifacts {
    /// The local Clifford operations that do the graph transformation.
    pub local_clifford_ops: *mut LcmhSingleQubitCliffordOperation,
    pub length_local_clifford_ops: usize,
    /// The costs of all the intermediate (accepted) graphs.
    pub costs: *mut f64,
    pub length_costs: usize,
}

mod vec_helper {
    use std::{mem::ManuallyDrop, ptr};

    pub struct VecHelper<T> {
        ptr: *mut T,
        length: usize,
    }

    impl<T> VecHelper<T> {
        /// # Safety
        ///
        /// The given pointer must be valid for `length` elements and must own the memory.
        pub unsafe fn frow_raw_parts(ptr: *mut T, length: usize) -> Self {
            VecHelper { ptr, length }
        }

        pub fn from_vec(vec: Vec<T>) -> Self {
            let boxed = ManuallyDrop::new(vec.into_boxed_slice());
            let ptr = boxed.as_ptr() as *mut T;
            let length = boxed.len();
            VecHelper { ptr, length }
        }

        pub fn get_ptr(&self) -> *mut T {
            self.ptr
        }

        pub fn get_length(&self) -> usize {
            self.length
        }
    }

    impl<T> Drop for VecHelper<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                // Safety: both possible creation methods ensure that the pointer is valid
                // for `length` elements and owns the memory.
                unsafe {
                    let _ = Box::from_raw(ptr::slice_from_raw_parts_mut(
                        self.ptr,
                        self.length,
                    ));
                }
            }
        }
    }
}
use vec_helper::VecHelper;

impl LcmhSearchArtifacts {
    fn from_artifacts(artifacts: SearchArtifacts) -> Self {
        let ops = ManuallyDrop::new(VecHelper::from_vec(artifacts.local_clifford_ops));
        let costs = ManuallyDrop::new(VecHelper::from_vec(artifacts.costs));
        Self {
            local_clifford_ops: ops.get_ptr(),
            length_local_clifford_ops: ops.get_length(),
            costs: costs.get_ptr(),
            length_costs: costs.get_length(),
        }
    }
}

impl Drop for LcmhSearchArtifacts {
    fn drop(&mut self) {
        // Safety: both possible creation methods ensure that the pointer is valid
        // for `length` elements and owns the memory.
        unsafe {
            let _ = VecHelper::frow_raw_parts(
                self.local_clifford_ops,
                self.length_local_clifford_ops,
            );
            let _ = VecHelper::frow_raw_parts(self.costs, self.length_costs);
        }
    }
}

/// Frees the given search artifacts.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_free_search_artifacts(artifacts: LcmhSearchArtifacts) {
    drop(artifacts);
}

/// The cost function (or energy function) in the LCMH search.
pub type CostFunction =
    extern "C" fn(graph: &LcmhGraph, num_single_qubit_lc_operations: usize) -> f64;

fn opt_seed(seed_from_entropy: bool, seed: u64) -> Option<u64> {
    if seed_from_entropy {
        None
    } else {
        Some(seed)
    }
}

/// Performs the search changing the graph in-place.
///
/// ...
///
/// # Arguments
/// - `graph`: The LCMH graph to be transformed.
/// - `cooling_config`: The cooling configuration that is used to control the search.
/// - `cost_function`: The cost function (or energy function) that is used to evaluate the
///   quality of the proposed graph. It gets passed the the proposed graph and the total
///   number of single-qubit Clifford gates that have been applied so far (including the
///   proposed step).
/// - `seed_from_entropy`: If true, the random number generator is seeded from entropy.
/// - `seed`: If `seed_from_entropy` is false, the random number generator is seeded with
///   this value.
///
/// # Returns
/// - Additional artifacts from the search.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_search(
    graph: &mut LcmhGraph,
    cooling_config: LcmhCoolingConfiguration,
    cost_function: CostFunction,
    seed_from_entropy: bool,
    seed: u64,
) -> LcmhSearchArtifacts {
    let artifacts = search::search(
        graph,
        cooling_config.into(),
        cost_function,
        opt_seed(seed_from_entropy, seed),
    );
    LcmhSearchArtifacts::from_artifacts(artifacts)
}

/// Perform the search ...
///
/// The input graph is encoded in the `slices`, cf.
/// [lcmh_transform_cabaliser_graph_to_lcmh_graph].
///
/// # Arguments
/// ...
///
/// # Returns
/// ...
///
/// # Safety
/// `slices` must be initialised for `n_qubits` and each slice `slices[i]` must again be
/// valid for `n_qubits`. Similarly, `output_slices` must be valid for `n_qubits` and each
/// slice `output_slices[i]` must again be valid for `n_qubits`.
#[unsafe(no_mangle)]
unsafe extern "C" fn lcmh_direct_search(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cooling_config: LcmhCoolingConfiguration,
    cost_function: CostFunction,
    seed_from_entropy: bool,
    seed: u64,
    output_slices: *mut *mut u64,
) -> LcmhSearchArtifacts {
    // Safety: this function assumes the same Safety guarantees `slices` as
    // [CabaliserGraph::new] for
    let c_graph = unsafe { CabaliserGraph::new(n_qubits, slices) };
    let mut graph = CabaliserGraph::to_graph(c_graph);
    let artifacts = search::search(
        &mut graph,
        cooling_config.into(),
        cost_function,
        opt_seed(seed_from_entropy, seed),
    );
    // Safety: this function assumes the same Safety guarantees `output_slices` as
    // [CabaliserGraph::from_graph] for `graph`
    unsafe { CabaliserGraph::from_graph(&graph, output_slices) };
    LcmhSearchArtifacts::from_artifacts(artifacts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
