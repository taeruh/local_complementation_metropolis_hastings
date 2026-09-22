pub use crate::graph::Graph;

pub enum VertexSelectionType {
    Random,
    Lexicographic,
}

pub fn search(
    // _graph: Graph,
    _num_steps: u32,
    // _vertex_selection_type: VertexSelectionType,
    cost_function: impl Fn(
        // num_vertices: u32,
        // num_edges: u32,
        // max_degree: u32,
        // num_executed_lcs: u32,
        u32,
        u32,
        u32,
        u32
    ) -> f64,
    //
) {
    println!("{:?}", cost_function(10, 20, 5, 3));
}
