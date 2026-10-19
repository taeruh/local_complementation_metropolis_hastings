use std::mem::ManuallyDrop;

use crate::{
    one_local_clifford::Clifford,
    search::{self, Cliffords, CoolingConfiguration, SearchArtifacts},
};

// On most platforms (appartly, all that are currently supported by Rust) C's size_t is
// equivalent to Rust's usize
// TODO: get the actual size_t via a build script...
#[allow(non_camel_case_types)]
pub type size_t = usize;

pub(crate) mod clifford_transformation {
    use crate::{c_interface::LcmhSingleQubitClifford, one_local_clifford::Clifford};

    // copy from cabaliser
    pub const LOCAL_CLIFFORD_MASK: LcmhSingleQubitClifford = 1 << 5;
    const _I_: LcmhSingleQubitClifford = LOCAL_CLIFFORD_MASK;
    const _X_: LcmhSingleQubitClifford = 0x01 | LOCAL_CLIFFORD_MASK;
    const _Y_: LcmhSingleQubitClifford = 0x02 | LOCAL_CLIFFORD_MASK;
    const _Z_: LcmhSingleQubitClifford = 0x03 | LOCAL_CLIFFORD_MASK;
    const _H_: LcmhSingleQubitClifford = 0x04 | LOCAL_CLIFFORD_MASK;
    const _S_: LcmhSingleQubitClifford = 0x05 | LOCAL_CLIFFORD_MASK;
    const _R_: LcmhSingleQubitClifford = 0x06 | LOCAL_CLIFFORD_MASK;
    const _HX_: LcmhSingleQubitClifford = 0x07 | LOCAL_CLIFFORD_MASK;
    const _SX_: LcmhSingleQubitClifford = 0x08 | LOCAL_CLIFFORD_MASK;
    const _RX_: LcmhSingleQubitClifford = 0x09 | LOCAL_CLIFFORD_MASK;
    const _HY_: LcmhSingleQubitClifford = 0x0a | LOCAL_CLIFFORD_MASK;
    const _HZ_: LcmhSingleQubitClifford = 0x0b | LOCAL_CLIFFORD_MASK;
    const _SH_: LcmhSingleQubitClifford = 0x0c | LOCAL_CLIFFORD_MASK;
    const _RH_: LcmhSingleQubitClifford = 0x0d | LOCAL_CLIFFORD_MASK;
    const _HS_: LcmhSingleQubitClifford = 0x0e | LOCAL_CLIFFORD_MASK;
    const _HR_: LcmhSingleQubitClifford = 0x0f | LOCAL_CLIFFORD_MASK;
    const _HSX_: LcmhSingleQubitClifford = 0x10 | LOCAL_CLIFFORD_MASK;
    const _HRX_: LcmhSingleQubitClifford = 0x11 | LOCAL_CLIFFORD_MASK;
    const _SHY_: LcmhSingleQubitClifford = 0x12 | LOCAL_CLIFFORD_MASK;
    const _RHY_: LcmhSingleQubitClifford = 0x13 | LOCAL_CLIFFORD_MASK;
    const _HSH_: LcmhSingleQubitClifford = 0x14 | LOCAL_CLIFFORD_MASK;
    const _HRH_: LcmhSingleQubitClifford = 0x15 | LOCAL_CLIFFORD_MASK;
    const _RHS_: LcmhSingleQubitClifford = 0x16 | LOCAL_CLIFFORD_MASK;
    const _SHR_: LcmhSingleQubitClifford = 0x17 | LOCAL_CLIFFORD_MASK;

    // translate some of them into our naming here
    const _SY_: LcmhSingleQubitClifford = _RX_;
    const _SZ_: LcmhSingleQubitClifford = _R_;
    const _SHX_: LcmhSingleQubitClifford = _RH_;
    const _SHZ_: LcmhSingleQubitClifford = _RHY_;
    const _HSY_: LcmhSingleQubitClifford = _HRX_;
    const _HSZ_: LcmhSingleQubitClifford = _HR_;
    const _HSHX_: LcmhSingleQubitClifford = _HRH_;
    const _HSHY_: LcmhSingleQubitClifford = _SHR_;
    const _HSHZ_: LcmhSingleQubitClifford = _RHS_;
    #[cfg(test)]
    pub fn check_translations() {
        let clifford_matrix_map =
            crate::one_local_clifford::tests::CliffordMatrixMap::new();

        let r = clifford_matrix_map.get_matrix(&Clifford::SZ);
        let h = clifford_matrix_map.get_matrix(&Clifford::H);
        let s = clifford_matrix_map.get_matrix(&Clifford::S);
        let y = clifford_matrix_map.get_matrix(&Clifford::Y);
        let x = clifford_matrix_map.get_matrix(&Clifford::X);

        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::SY)
                .is_equal_up_to_phase(&r.multiply(x))
        );
        assert!(clifford_matrix_map.get_matrix(&Clifford::SZ).is_equal_up_to_phase(r));
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::SHX)
                .is_equal_up_to_phase(&r.multiply(h))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::SHZ)
                .is_equal_up_to_phase(&r.multiply(h).multiply(y))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::HSY)
                .is_equal_up_to_phase(&h.multiply(r).multiply(x))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::HSZ)
                .is_equal_up_to_phase(&h.multiply(r))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::HSHX)
                .is_equal_up_to_phase(&h.multiply(r).multiply(h))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::HSHY)
                .is_equal_up_to_phase(&s.multiply(h).multiply(r))
        );
        assert!(
            clifford_matrix_map
                .get_matrix(&Clifford::HSHZ)
                .is_equal_up_to_phase(&r.multiply(h).multiply(s))
        );
    }

    impl Clifford {
        pub fn into_cabaliser_encoding(self) -> LcmhSingleQubitClifford {
            match self {
                Clifford::I => _I_,
                Clifford::X => _X_,
                Clifford::Y => _Y_,
                Clifford::Z => _Z_,
                Clifford::S => _S_,
                Clifford::SX => _SX_,
                Clifford::SY => _SY_,
                Clifford::SZ => _SZ_,
                Clifford::H => _H_,
                Clifford::HX => _HX_,
                Clifford::HY => _HY_,
                Clifford::HZ => _HZ_,
                Clifford::SH => _SH_,
                Clifford::SHX => _SHX_,
                Clifford::SHY => _SHY_,
                Clifford::SHZ => _SHZ_,
                Clifford::HS => _HS_,
                Clifford::HSX => _HSX_,
                Clifford::HSY => _HSY_,
                Clifford::HSZ => _HSZ_,
                Clifford::HSH => _HSH_,
                Clifford::HSHX => _HSHX_,
                Clifford::HSHY => _HSHY_,
                Clifford::HSHZ => _HSHZ_,
            }
        }
        pub fn from_cabaliser_encoding(c: LcmhSingleQubitClifford) -> Self {
            match c {
                _I_ => Clifford::I,
                _X_ => Clifford::X,
                _Y_ => Clifford::Y,
                _Z_ => Clifford::Z,
                _S_ => Clifford::S,
                _SX_ => Clifford::SX,
                _SY_ => Clifford::SY,
                _SZ_ => Clifford::SZ,
                _H_ => Clifford::H,
                _HX_ => Clifford::HX,
                _HY_ => Clifford::HY,
                _HZ_ => Clifford::HZ,
                _SH_ => Clifford::SH,
                _SHX_ => Clifford::SHX,
                _SHY_ => Clifford::SHY,
                _SHZ_ => Clifford::SHZ,
                _HS_ => Clifford::HS,
                _HSX_ => Clifford::HSX,
                _HSY_ => Clifford::HSY,
                _HSZ_ => Clifford::HSZ,
                _HSH_ => Clifford::HSH,
                _HSHX_ => Clifford::HSHX,
                _HSHY_ => Clifford::HSHY,
                _HSHZ_ => Clifford::HSHZ,
                _ => unreachable!("unexpected encoding: {c}"),
            }
        }
    }
}

mod graph_transformation {
    use std::ops::{BitOrAssign, BitXorAssign};

    use super::size_t;
    use crate::graph::Graph;

    pub const CHUNK_SIZE: usize = 64;

    /// A helper struct to access that graph data structure as they are represented in
    /// Cabaliser.
    #[cfg_attr(test, derive(Clone))]
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
            // columns which is initialised as [Self::new] guarantees that
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

        #[cfg(test)]
        // the safety arguments are analougous to [Self::to_graph]
        pub fn simpler_to_graph(c_graph: CabaliserGraph) -> Graph {
            let mut graph = Graph::new(c_graph.n_qubits);
            for i in 0..c_graph.n_qubits {
                let slice = unsafe { *c_graph.slices.add(i) };
                for j in 0..c_graph.n_qubits {
                    let chunk = unsafe { *slice.add(j / CHUNK_SIZE) };
                    if (chunk & (1 << (j % CHUNK_SIZE))) != 0 {
                        graph.add_edge(i, j);
                    }
                }
            }
            graph
        }

        /// Writes the given [Graph] into the given `buffer_slices` that is assumed to be
        /// initalised with zeros.
        ///
        /// # Safety
        /// `buffer_slices` must be valid for `graph.num_nodes()` and each slice
        /// `buffer_slices[i]` must again be valid for `graph.num_nodes()`.
        pub unsafe fn from_graph(
            graph: &Graph,
            buffer_slices: *mut *mut u64,
            original_graph: Option<&Graph>,
        ) -> CabaliserGraph {
            let c_graph = CabaliserGraph {
                n_qubits: graph.num_nodes(),
                slices: buffer_slices,
            };
            if let Some(original_graph) = original_graph {
                // cf. below for the safety arguments
                for i in 0..graph.num_nodes() {
                    let slice = unsafe { *c_graph.slices.add(i) };
                    for &neighbour in original_graph.get_neighbours(i).unwrap() {
                        let chunk = unsafe { &mut *slice.add(neighbour / CHUNK_SIZE) };
                        chunk.bitxor_assign(1 << (neighbour % CHUNK_SIZE))
                    }
                }
            }
            for i in 0..graph.num_nodes() {
                // Safety: we have i < graph.num_nodes() <= graph.num_nodes - 1, which is
                // a valid index
                let slice = unsafe { *c_graph.slices.add(i) };
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

pub(crate) use graph_transformation::CabaliserGraph;

/// The graph encoding we use for the LCMH search.
///
/// # Garbage collection
/// Use [lcmh_free_lcmh_graph] to free the according memory.
pub type LcmhGraph = crate::graph::Graph;

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
    pub betas: *const f64,
    /// The number of "steps" per beta. Note that each "step" consists of
    /// `graph.num_nodes()` local complementation draws (not necessarily accepted).
    pub num_steps_per_beta: *const usize,
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
/// (i.e., instruction_t).
pub type LcmhSingleQubitClifford = u8;

/// Collection of search artifacts (apart from the transformed graph) that are returned by
/// the search functions.
///
/// # Garbage collection
/// Use [lcmh_free_search_artifacts] to free the according memory.
#[repr(C)]
pub struct LcmhSearchArtifacts {
    /// The costs of all the intermediate (accepted) graphs.
    pub costs: *mut f64,
    pub length_costs: usize,
    pub initial_cost: f64,
    pub best_cost: f64,
}

mod vec_helper {
    use std::{mem::ManuallyDrop, ptr};

    pub struct VecHelper<T> {
        ptr: *mut T,
        length: usize,
    }

    impl<T> VecHelper<T> {
        /// # Safety
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
        let costs = ManuallyDrop::new(VecHelper::from_vec(artifacts.costs));
        Self {
            costs: costs.get_ptr(),
            length_costs: costs.get_length(),
            initial_cost: artifacts.initial_cost,
            best_cost: artifacts.best_cost,
        }
    }
}

impl Drop for LcmhSearchArtifacts {
    fn drop(&mut self) {
        // Safety: both possible creation methods ensure that the pointer is valid
        // for `length` elements and owns the memory.
        unsafe {
            let _ = VecHelper::frow_raw_parts(self.costs, self.length_costs);
        }
    }
}

/// Frees the given search artifacts.
#[unsafe(no_mangle)]
pub extern "C" fn lcmh_free_search_artifacts(artifacts: LcmhSearchArtifacts) {
    // println!("Freeing search artifacts");
    drop(artifacts);
}

/// The cost function (or energy function) in the LCMH search.
///
/// There are functions to get data from the graph, e.g. [lcmh_get_num_edges].
pub type LcmhCostFunction = extern "C" fn(graph: &LcmhGraph) -> f64;

fn opt_seed(seed_from_entropy: bool, seed: u64) -> Option<u64> {
    if seed_from_entropy {
        None
    } else {
        Some(seed)
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn search_helper(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed: Option<u64>,
) -> LcmhSearchArtifacts {
    // Safety: this function assumes the same Safety guarantees `slices` as
    // [CabaliserGraph::new] for
    let c_graph = unsafe { CabaliserGraph::new(n_qubits, slices) };
    let original_graph = CabaliserGraph::to_graph(c_graph);
    let mut base_cliffords = Vec::with_capacity(n_qubits);
    for i in 0..n_qubits {
        // Safety: corrections has a length of n_qubits, so i < n_qubits
        let c_clifford = unsafe { *cliffords.add(i) };
        base_cliffords.push(Clifford::from_cabaliser_encoding(c_clifford));
    }
    let (graph, corrections, artifacts) = search::search(
        original_graph.clone(),
        Cliffords::new(base_cliffords),
        cooling_config,
        cost_function,
        seed,
    );
    // Safety: this function assumes the same Safety guarantees `output_slices` as
    // [CabaliserGraph::from_graph] for `graph`.
    // unsafe { CabaliserGraph::from_graph(&graph, output_slices, Some(&original_graph)) };
    unsafe { CabaliserGraph::from_graph(&graph, output_slices, None) };
    for (i, clifford) in corrections.into_data().into_iter().enumerate() {
        // Safety: corrections has a length of n_qubits, so i < n_qubits
        unsafe { *output_cliffords.add(i) = clifford.into_cabaliser_encoding() };
    }
    LcmhSearchArtifacts::from_artifacts(artifacts)
}

/// Perform the search ...
///
/// The input graph is encoded in the `slices`, cf.
/// [lcmh_transform_cabaliser_graph_to_lcmh_graph].
///
/// # Arguments
/// - `n_qubits`: The number of qubits/nodes in the graph.
/// - `slices`: A pointer to the pointers that point to the column of the graph (cf. graph
///   representation in Cabaliser).
/// - `cliffords`: A pointer to the original local Clifford corrections (they are assumed
///   to be either I or H).
/// - `output_slices`: A pointer to the pointers that point to the column of a
///   *zero-initialised* buffer that is used to write the transformed graph into (cf. graph
///   representation).
/// - `output_cliffords`: A pointer to a buffer that is used to write the transformed
///   local Clifford corrections into.
/// - `cooling_config`: The cooling configurations that are used for the MH search.
/// - `cost_function`: The cost function (or energy function) that is used in the MH
///   search.
/// - `seed_from_entropy`: If true, the random number generator is seeded from entropy,
///   otherwise the seed` is used.
/// - `seed`: The seed for the random number generator if `seed_from_entropy` is false.
///
/// # Returns
/// - Additional artifacts from the search, e.g., the cost for each accepted step.
///
/// # Safety
/// - `slices` must be initialised for `n_qubits` and each slice `slices[i]` must again be
///   valid for `n_qubits`. Similarly, `output_slices` must be valid for `n_qubits` and
///   each slice `output_slices[i]` must again be valid for `n_qubits`.
/// - `cliffords` and `output_cliffords` must be valid for `n_qubits`.
/// - The invariants of [LcmhCoolingConfiguration] must hold.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
    cooling_config: LcmhCoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed_from_entropy: bool,
    seed: u64,
) -> LcmhSearchArtifacts {
    unsafe {
        search_helper(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
            cooling_config.into(),
            cost_function,
            opt_seed(seed_from_entropy, seed),
        )
    }
}

/// Same as [lcmh_search] but does not return any artifacts.
///
/// # Safety
/// Sam as for [lcmh_search].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search_no_artifacts(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
    cooling_config: LcmhCoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed_from_entropy: bool,
    seed: u64,
) {
    unsafe {
        let _ = lcmh_search(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
            cooling_config,
            cost_function,
            seed_from_entropy,
            seed,
        );
    }
}

extern "C" fn default_cost_function(graph: &LcmhGraph) -> f64 {
    graph.num_edges() as f64
}

fn get_default_cooling_config() -> CoolingConfiguration {
    // not sure what a good default cooling schedule is ...
    CoolingConfiguration {
        betas: vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 1.0, 10.0, 100.0, 1000.0, 10000.0],
        num_steps_per_beta: vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20],
    }
}

/// Same as [lcmh_search] but with defaults.
///
/// # Safety
/// Sam as for [lcmh_search].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search_with_defaults(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
) -> LcmhSearchArtifacts {
    unsafe {
        search_helper(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
            get_default_cooling_config(),
            default_cost_function,
            None,
        )
    }
}

/// Same as [lcmh_search] but with defaults and no artifacts returned.
///
/// # Safety
/// Sam as for [lcmh_search].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search_with_defaults_no_return(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
) {
    unsafe {
        let _ = lcmh_search_with_defaults(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
        );
    }
}

/// Same as [lcmh_search] but with defaults, except for the cooling configuration.
///
/// # Safety
/// Sam as for [lcmh_search].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search_with_defaults_except_cooling_config(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
    cooling_config: LcmhCoolingConfiguration,
) -> LcmhSearchArtifacts {
    unsafe {
        search_helper(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
            cooling_config.into(),
            default_cost_function,
            None,
        )
    }
}

/// Same as [lcmh_search] but with defaults, except for the cooling configuration, and no
/// artifacts returned.
///
/// # Safety
/// Sam as for [lcmh_search].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_search_with_defaults_except_cooling_config_no_return(
    n_qubits: size_t,
    slices: *mut *mut u64,
    cliffords: *mut LcmhSingleQubitClifford,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
    cooling_config: LcmhCoolingConfiguration,
) {
    unsafe {
        let _ = lcmh_search_with_defaults_except_cooling_config(
            n_qubits,
            slices,
            cliffords,
            output_slices,
            output_cliffords,
            cooling_config,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        c_interface::graph_transformation::CHUNK_SIZE, one_local_clifford::Clifford,
    };

    #[test]
    fn check_translations() {
        super::clifford_transformation::check_translations();
    }

    #[test]
    fn invert_encoding() {
        for cliff in 0..24 {
            let clifford = Clifford::from_discriminant(cliff);
            assert_eq!(
                clifford,
                Clifford::from_cabaliser_encoding(clifford.into_cabaliser_encoding())
            );
            let cliff = cliff | super::clifford_transformation::LOCAL_CLIFFORD_MASK;
            assert_eq!(
                cliff,
                Clifford::from_cabaliser_encoding(cliff).into_cabaliser_encoding()
            );
        }
    }

    #[test]
    fn graph_tranformations() {
        crate::tests::catch_seed_on_panic(None, |mut rng| {
            let (graph, _) = crate::tests::create_random_graph(&mut rng, 50);
            let num_nodes = graph.num_nodes();
            #[allow(clippy::manual_div_ceil)]
            let chunks_per_column = (num_nodes + CHUNK_SIZE - 1) / CHUNK_SIZE;
            let slices_buffer = vec![0.0; num_nodes * chunks_per_column];
            let (slices_buffer_ptr, slices_buffer_len, slices_buffer_cap) =
                slices_buffer.into_raw_parts();
            let mut buffer_slices: Vec<*mut u64> = Vec::with_capacity(num_nodes);
            for i in 0..num_nodes {
                let slice_ptr =
                    unsafe { slices_buffer_ptr.add(i * chunks_per_column) as *mut u64 };
                buffer_slices.push(slice_ptr);
            }
            let buffer_slices_ptr = buffer_slices.as_mut_ptr();

            let c_graph = unsafe {
                graph_transformation::CabaliserGraph::from_graph(
                    &graph,
                    buffer_slices_ptr,
                    None,
                )
            };
            let graph_from_c_graph =
                graph_transformation::CabaliserGraph::simpler_to_graph(c_graph.clone());
            assert_eq!(graph, graph_from_c_graph);
            let graph_from_c_graph =
                graph_transformation::CabaliserGraph::to_graph(c_graph);
            assert_eq!(graph, graph_from_c_graph);
            let _ = unsafe {
                Vec::from_raw_parts(
                    slices_buffer_ptr,
                    slices_buffer_len,
                    slices_buffer_cap,
                )
            };
        });
    }
}
