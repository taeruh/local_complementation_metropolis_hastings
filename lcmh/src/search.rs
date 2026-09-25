use rand::{RngExt, SeedableRng, distr::StandardUniform, rngs::SysRng};
use rand_pcg::Pcg64Mcg;

use crate::{
    graph::Graph,
    c_interface::{LcmhCostFunction, LcmhSingleQubitCliffordOperation},
};

// the local complementation cliffords are S^3 and HSH; in cababliser we have the _R_ =
// _S__Z_, as well as _HSH_ therefore, S^3 -> _R_ and HSH -> _HSH_
mod clifford_ops {
    use crate::c_interface::LcmhSingleQubitClifford;

    const LOCAL_CLIFFORD_MASK: LcmhSingleQubitClifford = 1 << 5;
    const _R_: LcmhSingleQubitClifford = 0x06 | LOCAL_CLIFFORD_MASK;
    const _HSH_: LcmhSingleQubitClifford = 0x14 | LOCAL_CLIFFORD_MASK;
    pub const OP_NODE: LcmhSingleQubitClifford = _R_;
    pub const OP_NEIGHBOUR: LcmhSingleQubitClifford = _HSH_;
}

pub struct CoolingConfiguration {
    pub betas: Vec<f64>,
    pub num_steps_per_beta: Vec<usize>,
}

#[derive(Debug)]
pub struct SearchArtifacts {
    pub local_clifford_ops: Vec<LcmhSingleQubitCliffordOperation>,
    pub costs: Vec<f64>,
}

pub fn search(
    graph: &mut Graph,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed: Option<u64>,
) -> SearchArtifacts {
    let mut rng = match seed {
        Some(seed) => Pcg64Mcg::seed_from_u64(seed),
        None => Pcg64Mcg::try_from_rng(&mut SysRng).expect(
            "Failed to create random number
            generator from entropy",
        ),
    };

    let mut artifacts = SearchArtifacts {
        local_clifford_ops: Vec::new(),
        costs: Vec::new(),
    };

    let mut current_cost = cost_function(graph, 0);
    artifacts.costs.push(current_cost);

    for (&beta, &num_steps) in cooling_config
        .betas
        .iter()
        .zip(cooling_config.num_steps_per_beta.iter())
    {
        for _ in 0..num_steps {
            current_cost = mc_step(
                graph,
                cost_function,
                current_cost,
                beta,
                &mut artifacts,
                &mut rng,
            );
        }
    }

    artifacts
}

fn mc_step(
    graph: &mut Graph,
    cost_function: LcmhCostFunction,
    beta: f64,
    mut current_cost: f64,
    artifacts: &mut SearchArtifacts,
    rng: &mut Pcg64Mcg,
) -> f64 {
    let num_nodes = graph.num_nodes();
    for _ in 0..num_nodes {
        let node = rng.random_range(0..num_nodes);
        let num_neighbours = graph.get_neighbours(node).unwrap().len();
        graph.local_complementation(node);
        let new_cost =
            cost_function(graph, artifacts.local_clifford_ops.len() + num_neighbours + 1);
        let delta_cost = new_cost - current_cost;
        if delta_cost < 0.0
            || rng.sample::<f64, StandardUniform>(StandardUniform)
                < (-beta * delta_cost).exp()
        {
            current_cost = new_cost;
            artifacts.local_clifford_ops.push(LcmhSingleQubitCliffordOperation {
                node,
                operation: clifford_ops::OP_NODE,
            });
            for neighbour in graph.get_neighbours(node).unwrap() {
                artifacts.local_clifford_ops.push(LcmhSingleQubitCliffordOperation {
                    node: *neighbour,
                    operation: clifford_ops::OP_NEIGHBOUR,
                });
            }
            artifacts.costs.push(current_cost);
        } else {
            graph.local_complementation(node);
        }
    }
    current_cost
}

#[cfg(test)]
mod tests {

    use super::*;

    extern "C" fn test_cost_function(
        graph: &Graph,
        _num_single_qubit_lc_operations: usize,
    ) -> f64 {
        use crate::c_interface::lcmh_get_num_edges;
        lcmh_get_num_edges(graph) as f64
    }

    #[test]
    fn test_search() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        println!("{:?}", graph);

        let cooling_config = CoolingConfiguration {
            betas: vec![0.1, 0.5, 10.0],
            num_steps_per_beta: vec![5, 5, 5],
        };

        let artifacts = search(&mut graph, cooling_config, test_cost_function, None);

        println!("{:?}", graph);
        println!("{:?}", artifacts.costs);
    }
}
