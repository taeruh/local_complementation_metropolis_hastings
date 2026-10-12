use crate::{
    c_interface::{CabaliserGraph, LcmhSingleQubitClifford, size_t},
    graph::Graph,
    search::{self, CoolingConfiguration},
};

extern "C" fn cost_function(graph: &Graph) -> f64 {
    graph.num_edges() as f64
}

/// Simplified version of [lcmh_direct_search](crate::c_interface::lcmh_direct_search).
///
/// # Safety
/// `slices` must be initialised for `n_qubits` and each slice `slices[i]` must again be
/// valid for `n_qubits`. Similarly, `output_slices` must be valid for `n_qubits` and each
/// slice `output_slices[i]` must again be valid for `n_qubits`. `output_cliffords` must
/// be valid for `n_qubits`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcmh_simpler_direct_search(
    n_qubits: size_t,
    slices: *mut *mut u64,
    output_slices: *mut *mut u64,
    output_cliffords: *mut LcmhSingleQubitClifford,
) {
    // not sure what a good default cooling schedule is ...
    let cooling_config = CoolingConfiguration {
        betas: vec![100.0, 1000.0, 10000.0, 100000.0, 1000000.0],
        num_steps_per_beta: vec![20, 50, 100, 100, 100],
    };

    let c_graph = unsafe { CabaliserGraph::new(n_qubits, slices) };
    let mut graph = CabaliserGraph::to_graph(c_graph);
    // println!("{:?}", graph);
    let artifacts = search::search(&mut graph, cooling_config, cost_function, None);
    // println!("{:?}", graph);
    // println!("{:?}", artifacts.costs);
    // println!("{:?}", artifacts.costs.first().unwrap());
    // println!("{:?}", artifacts.costs.last().unwrap());

    unsafe { CabaliserGraph::from_graph(&graph, output_slices) };
    for (i, clifford) in artifacts
        .clifford_stacks
        .into_data()
        .into_iter()
        .map(|stack| stack.into_clifford().into_cabaliser_encoding())
        .enumerate()
    {
        // println!("{:?}", (i, clifford));
        unsafe { *output_cliffords.add(i) = clifford };
    }
}
