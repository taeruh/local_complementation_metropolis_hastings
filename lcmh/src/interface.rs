pub use crate::graph::Graph;
use crate::search::{
    self,
    LocalOperations, // TODO: actually use cabalisers clifford interface
};

pub enum VertexSelectionType {
    Random,
    Lexicographic,
}

// On most platforms (appartly, all that are currently supported by Rust) C's size_t is
// equivalent to Rust's usize
// TODO: get the actual size_t via a build script...
#[allow(non_camel_case_types)]
type size_t = usize;

const CHUNK_SIZE: usize = 64;

struct CabaliserGraph {
    n_qubits: size_t,
    // the graph is stored as adjacency matrix; the matrix is stored as single bit-vector,
    // however, access to it is given via these slices, where each slice corresponds to a
    // row of the adjacency matrix and via the slice entries one access the individual
    // 64-bit chunks of the row
    slices: *mut *mut u64,
}

impl CabaliserGraph {
    /// # Safety
    /// The slice(s) must be valid for reads of `n_qubits`
    unsafe fn to_graph(c_graph: CabaliserGraph) -> Graph {
        if c_graph.n_qubits == 0 {
            return Graph::new(0);
        }

        let num_chunks = c_graph.n_qubits.div_ceil(CHUNK_SIZE);
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
            let slice = unsafe { *c_graph.slices.add(i) };
            let start_j = i + 1;
            let current_chunk_idx = start_j / CHUNK_SIZE;
            if current_chunk_idx != num_chunks - 1 {
                let chunk = unsafe { *slice.add(current_chunk_idx) };
                for j in start_j..CHUNK_SIZE {
                    potentially_add_edge(chunk, current_chunk_idx, i, j);
                }
            }
            for chunk_idx in (current_chunk_idx + 1)..(num_chunks - 1) {
                let chunk = unsafe { *slice.add(chunk_idx) };
                for j in 0..CHUNK_SIZE {
                    potentially_add_edge(chunk, chunk_idx, i, j);
                }
            }
            let chunk = unsafe { *slice.add(last_chunk_idx) };
            for j in (start_j % CHUNK_SIZE)..(max_bit_in_last_chunk + 1) {
                potentially_add_edge(chunk, last_chunk_idx, i, j);
            }
        }

        graph
    }

    unsafe fn from_graph(
        graph: &Graph,
        // need to be valid for `graph.num_nodes()` and initialised to zeros
        slices_buffer: *mut *mut u64,
    ) -> CabaliserGraph {
        let c_graph = CabaliserGraph {
            n_qubits: graph.num_nodes(),
            slices: slices_buffer,
        };
        for i in 0..graph.num_nodes() {
            let slice = unsafe { *slices_buffer.add(i) };
            for &neighbour in graph.get_neighbours(i).unwrap() {
                unsafe {
                    *slice.add(neighbour / CHUNK_SIZE) |= 1 << (neighbour % CHUNK_SIZE);
                }
            }
        }
        c_graph
    }
}

#[repr(C)]
struct SearchArtifacts {
    // ...
}

#[unsafe(no_mangle)]
unsafe extern "C" fn direct_search(
    n_qubits: size_t,
    slices: *mut *mut u64,
    num_steps: usize,
    cost_function: extern "C" fn(
        num_vertices: usize,
        num_edges: usize,
        max_degree: usize,
        num_executed_lcs: usize,
    ) -> f64,
    // must be valid for `n_qubits` and initialised to zeros
    output_slices: *mut *mut u64,
) -> SearchArtifacts {
    let c_graph = CabaliserGraph { n_qubits, slices };
    let mut graph = unsafe { CabaliserGraph::to_graph(c_graph) };
    let (ops, num_edges, max_degree, cost) =
        search::search(&mut graph, num_steps, cost_function);
    unsafe { CabaliserGraph::from_graph(&graph, output_slices) };
    SearchArtifacts {
        // ...
    }
}

#[repr(C)]
struct LcmhGraph {
    graph: *mut Graph,
}

impl LcmhGraph {
    fn new(graph: Graph) -> Self {
        let boxed_graph = Box::new(graph);
        let graph_ptr = Box::into_raw(boxed_graph);
        LcmhGraph { graph: graph_ptr }
    }

    fn get_graph(&self) -> &Graph {
        unsafe { &*self.graph }
    }

    fn get_graph_mut(&mut self) -> &mut Graph {
        unsafe { &mut *self.graph }
    }
}

impl Drop for LcmhGraph {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.graph));
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn transform_cabaliser_graph_to_lcmh_graph(
    n_qubits: size_t,
    slices: *mut *mut u64,
) -> LcmhGraph {
    let c_graph = CabaliserGraph { n_qubits, slices };
    LcmhGraph::new(unsafe { CabaliserGraph::to_graph(c_graph) })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn transform_lcmh_graph_to_cabaliser_graph(
    graph: &LcmhGraph,
    // must be valid for `graph.num_nodes()` and initialised to zeros
    slices_buffer: *mut *mut u64,
) {
    let graph = graph.get_graph();
    unsafe { CabaliserGraph::from_graph(graph, slices_buffer) };
}

#[unsafe(no_mangle)]
extern "C" fn clone_lcmh_graph(graph: &LcmhGraph) -> LcmhGraph {
    LcmhGraph::new(graph.get_graph().clone())
}

#[unsafe(no_mangle)]
extern "C" fn free_lcmh_graph(graph: LcmhGraph) {
    drop(graph);
}

#[unsafe(no_mangle)]
extern "C" fn search(
    graph: &mut LcmhGraph,
    num_steps: usize,
    cost_function: extern "C" fn(
        num_vertices: usize,
        num_edges: usize,
        max_degree: usize,
        num_executed_lcs: usize,
    ) -> f64,
) -> SearchArtifacts {
    let (ops, num_edges, max_degree, cost) =
        search::search(graph.get_graph_mut(), num_steps, cost_function);
    SearchArtifacts {
        // ...
    }
}
