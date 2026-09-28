use std::collections::HashSet;

use rand::{RngExt, SeedableRng, distr::StandardUniform, rngs::SysRng};
use rand_pcg::Pcg64Mcg;

use crate::{c_interface::LcmhCostFunction, clifford_stack::CliffordStack, graph::Graph};

pub struct CoolingConfiguration {
    pub betas: Vec<f64>,
    pub num_steps_per_beta: Vec<usize>,
}

#[cfg_attr(test, derive(Debug))]
pub struct CliffordStacks {
    stacks: Vec<CliffordStack>,
    #[cfg(test)]
    pub naive_num_lc_ops: Vec<usize>,
}

#[cfg_attr(test, derive(Debug))]
pub struct SearchArtifacts {
    pub clifford_stacks: CliffordStacks,
    pub costs: Vec<f64>,
}

impl CliffordStacks {
    pub fn into_data(self) -> Vec<CliffordStack> {
        self.stacks
    }

    pub fn local_complementation(&mut self, node: usize, neighbours: &HashSet<usize>) {
        self.stacks[node].push_r();
        for neighbour in neighbours {
            self.stacks[*neighbour].push_hsh();
        }
        #[cfg(test)]
        {
            self.naive_num_lc_ops[node] += neighbours.len() + 1;
        }
    }

    pub fn inverse_local_complementation(
        &mut self,
        node: usize,
        neighbours: &HashSet<usize>,
    ) {
        self.stacks[node].inverse_push_r();
        for neighbour in neighbours {
            self.stacks[*neighbour].inverse_push_hsh();
        }
        #[cfg(test)]
        {
            self.naive_num_lc_ops[node] -= neighbours.len() + 1;
        }
    }

    pub fn num_paulis_approx(&self) -> usize {
        self.stacks.len()
    }

    #[cfg(test)]
    pub fn num_pauli_exact(&self) -> usize {
        self.stacks.iter().filter(|stack| !stack.top_is_identity()).count()
    }

    pub fn num_non_pauli_cliffords(&self) -> usize {
        self.stacks.iter().map(|stack| stack.num_non_pauli_cliffords()).sum()
    }

    pub fn num_total_approx(&self) -> usize {
        self.num_paulis_approx() + self.num_non_pauli_cliffords()
    }

    #[cfg(test)]
    pub fn num_total_exact(&self) -> usize {
        self.num_pauli_exact() + self.num_non_pauli_cliffords()
    }
}

pub fn search(
    graph: &mut Graph,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed: Option<u64>,
) -> SearchArtifacts {
    let mut rng = match seed {
        Some(seed) => Pcg64Mcg::seed_from_u64(seed),
        None => Pcg64Mcg::try_from_rng(&mut SysRng)
            .expect("Failed to create random number generator from entropy"),
    };

    let mut artifacts = SearchArtifacts {
        clifford_stacks: CliffordStacks {
            stacks: Vec::from_iter((0..graph.num_nodes()).map(|_| CliffordStack::new())),
            #[cfg(test)]
            naive_num_lc_ops: vec![0; graph.num_nodes()],
        },
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
        graph.local_complementation(node);
        // the following will be inverted below if the step is rejected {{
        let neighbours = graph.get_neighbours(node).unwrap();
        artifacts.clifford_stacks.local_complementation(node, neighbours);
        // }}
        // TODO: ask whether we rather want to use num_total_exact
        let new_cost = cost_function(graph, artifacts.clifford_stacks.num_total_approx());
        let delta_cost = new_cost - current_cost;
        if delta_cost < 0.0
            || rng.sample::<f64, StandardUniform>(StandardUniform)
                < (-beta * delta_cost).exp()
        {
            current_cost = new_cost;
            artifacts.costs.push(current_cost);
        } else {
            artifacts
                .clifford_stacks
                .inverse_local_complementation(node, neighbours);
            graph.local_complementation(node);
        }
    }
    current_cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c_interface::LcmhSearchArtifacts;

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

        // println!("{:?}", graph);

        let cooling_config = CoolingConfiguration {
            betas: vec![0.1, 0.5, 10.0],
            num_steps_per_beta: vec![5, 5, 5],
        };

        let artifacts = search(&mut graph, cooling_config, test_cost_function, None);
        let naive_num_lc_ops =
            artifacts.clifford_stacks.naive_num_lc_ops.iter().sum::<usize>();
        let actual_num_lc_ops_approx = artifacts.clifford_stacks.num_total_approx();
        let actual_num_lc_ops_exact = artifacts.clifford_stacks.num_total_exact();
        println!("{:?}", (naive_num_lc_ops, actual_num_lc_ops_approx));

        let c_artifacts = LcmhSearchArtifacts::from_artifacts(artifacts);
        let c_lc_ops = c_artifacts.length_local_clifford_ops;
        println!("{:?}", (c_lc_ops, actual_num_lc_ops_exact));
    }
}
