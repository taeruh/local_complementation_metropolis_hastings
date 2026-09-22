use lcmh::interface;

#[unsafe(no_mangle)]
pub extern "C" fn search(
    num_steps: u32,
    cost_function: extern "C" fn(
        num_vertices: u32,
        num_edges: u32,
        max_degree: u32,
        num_executed_lcs: u32,
    ) -> f64,
) {
    let rust_cost_function = move |num_vertices: u32,
                                   num_edges: u32,
                                   max_degree: u32,
                                   num_executed_lcs: u32| {
        cost_function(num_vertices, num_edges, max_degree, num_executed_lcs)
    };
    interface::search(num_steps, rust_cost_function);
}
