use crate::graph::Graph;

pub struct LocalOperations {}

pub fn search(
    graph: &mut Graph,
    num_steps: usize,
    // _vertex_selection_type: VertexSelectionType,
    cost_function: extern "C" fn(
        num_vertices: usize,
        num_edges: usize,
        max_degree: usize,
        num_executed_lcs: usize,
    ) -> f64,
    //
) -> (Vec<LocalOperations>, usize, usize, f64) {
    println!("{:?}", cost_function(10, 20, 5, 3));
    (Vec::new(), 0, 0, 0.0)
}
