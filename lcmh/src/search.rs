use crate::graph::Graph;

// the local complementation cliffords are S^3 and HSH; in cababliser we have the _R_ =
// _S__Z_, as well as _HSH_ therefore, S^3 -> _R_ and HSH -> _HSH_
pub type LocalClifford = u8;
const LOCAL_CLIFFORD_MASK: LocalClifford = 1 << 5;
const _R_: LocalClifford = 0x06 | LOCAL_CLIFFORD_MASK;
const _HSH_: LocalClifford = 0x14 | LOCAL_CLIFFORD_MASK;
const OP_NODE: LocalClifford = _R_;
const OP_NEIGHBOUR: LocalClifford = _HSH_;

pub struct CoolingConfiguration {
    pub num_betas: usize,
    pub betas: Vec<f64>,
    pub num_steps_per_beta: Vec<usize>,
}

pub struct SearchArtifacts {
    pub local_clifford_ops: Vec<LocalClifford>,
    pub cost: f64,
}

pub fn search(
    graph: &mut Graph,
    cooling_config: CoolingConfiguration,
    cost_function: extern "C" fn(
        graph: *mut Graph,
        num_single_qubit_lc_operations: usize,
    ) -> f64,
    seed: Option<u64>,
) -> SearchArtifacts {
    println!("{:?}", cost_function(graph, 3));
    SearchArtifacts {
        local_clifford_ops: vec![],
        cost: 0.0,
    }
}
