use rand::{RngExt, distr::StandardUniform};
use rand_pcg::Pcg64Mcg;

use crate::{
    c_interface::LcmhCostFunction,
    graph::{Graph, HashSet},
    one_local_clifford::Clifford,
    stabiliser_group,
};

#[derive(Debug)]
pub struct CoolingConfiguration {
    pub betas: Vec<f64>,
    pub num_steps_per_beta: Vec<usize>,
}

// TODO: make this here generic so that we don't have to copy when it comes from C
#[derive(Clone, Debug)]
pub struct Cliffords {
    cliffords: Vec<Clifford>,
}

#[cfg_attr(test, derive(Debug))]
pub struct SearchArtifacts {
    pub costs: Vec<f64>,
    pub initial_cost: f64,
    pub best_cost: f64,
}

impl Cliffords {
    pub fn new(cliffords: Vec<Clifford>) -> Self {
        Self { cliffords }
    }

    pub fn get_data(&self) -> &Vec<Clifford> {
        &self.cliffords
    }

    pub fn into_data(self) -> Vec<Clifford> {
        self.cliffords
    }

    // // never actually need it
    //
    // // assuming the graph is encoded on the z-block
    // pub fn local_complementation(&mut self, node: usize, neighbours: &HashSet<usize>) {
    //     self.cliffords[node].multiply_hsh_from_left();
    //     for neighbour in neighbours {
    //         self.cliffords[*neighbour].multiply_sz_from_left();
    //     }
    // }

    // assuming the graph is encoded on the z-block
    pub fn inverse_local_complementation(
        &mut self,
        node: usize,
        neighbours: &HashSet<usize>,
    ) {
        self.cliffords[node].multiply_hshx_from_right();
        for neighbour in neighbours {
            self.cliffords[*neighbour].multiply_s_from_right();
        }
    }
}

struct Search {
    graph: Graph,
    cliffords: Cliffords,
    best_graph: Graph,
    best_cliffords: Cliffords,
    best_cost: f64,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    rng: Pcg64Mcg,
    artifacts: SearchArtifacts,
}

fn check_search_result(
    graph: &Graph,
    cliffords: &Cliffords,
    best_graph: &Graph,
    best_cliffords: &Cliffords,
) {
    println!("Checking search result");
    println!("Initial graph:\n{graph:?}");
    println!("Initial cliffords:\n{cliffords:?}");
    println!("Best graph:\n{best_graph:?}");
    println!("Best cliffords:\n{best_cliffords:?}");
    let reduced_init = stabiliser_group::reduce(graph, cliffords.get_data());
    let reduced_after = stabiliser_group::reduce(best_graph, best_cliffords.get_data());
    assert_eq!(reduced_init, reduced_after);
}

pub fn search(
    graph: Graph,
    cliffords: Cliffords,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed: Option<u64>,
) -> (Graph, Cliffords, SearchArtifacts) {
    let mut search = Search::new(graph.clone(), cliffords.clone(), cooling_config, cost_function, seed);
    search.search();
    check_search_result(
        &graph,
        &cliffords,
        &search.best_graph,
        &search.best_cliffords,
    );
    (search.best_graph, search.best_cliffords, search.artifacts)
}

impl Search {
    fn new(
        graph: Graph,
        cliffords: Cliffords,
        cooling_config: CoolingConfiguration,
        cost_function: LcmhCostFunction,
        seed: Option<u64>,
    ) -> Self {
        let (rng, _) = crate::create_rng(seed);
        let current_cost = cost_function(&graph);
        let mut artifacts = SearchArtifacts {
            costs: Vec::new(),
            initial_cost: current_cost,
            best_cost: current_cost,
        };
        artifacts.costs.push(current_cost);
        let best_cost = current_cost;
        Self {
            best_graph: graph.clone(),
            best_cliffords: cliffords.clone(),
            best_cost,
            graph,
            cliffords,
            cooling_config,
            cost_function,
            rng,
            artifacts,
        }
    }

    fn search(&mut self) {
        let mut current_cost = *self.artifacts.costs.last().unwrap();
        for (beta, num_steps) in self
            .cooling_config
            .betas
            .clone()
            .into_iter()
            .zip(self.cooling_config.num_steps_per_beta.clone())
        {
            for _ in 0..num_steps {
                current_cost = self.mc_step(beta, current_cost);
            }
        }
        self.artifacts.best_cost = self.best_cost;
    }

    fn mc_step(&mut self, beta: f64, mut current_cost: f64) -> f64 {
        let num_nodes = self.graph.num_nodes();
        for _ in 0..num_nodes {
            let node = self.rng.random_range(0..num_nodes);
            // this here will be inverted below if the step is rejected
            self.graph.local_complementation(node);
            let new_cost = (self.cost_function)(&self.graph);
            let delta_cost = new_cost - current_cost;
            if delta_cost < 0.0
                || self.rng.sample::<f64, StandardUniform>(StandardUniform)
                    < (-beta * delta_cost).exp()
            {
                current_cost = new_cost;
                self.artifacts.costs.push(current_cost);
                self.cliffords.inverse_local_complementation(
                    node,
                    self.graph.get_neighbours(node).unwrap(),
                );
                // println!("{:?}", node);
                if current_cost < self.best_cost {
                    // println!("here");
                    self.best_cost = current_cost;
                    self.best_graph = self.graph.clone();
                    self.best_cliffords = self.cliffords.clone();
                }
            } else {
                self.graph.local_complementation(node);
            }
        }
        current_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stabiliser_group;

    extern "C" fn test_cost_function(graph: &Graph) -> f64 {
        use crate::c_interface::lcmh_get_num_edges;
        lcmh_get_num_edges(graph) as f64
    }

    #[test]
    fn test_search() {
        crate::tests::catch_seed_on_panic(None, |mut rng| {
            let (graph, cliffords) = crate::tests::create_random_graph(&mut rng, 30);
            let reduced_init = stabiliser_group::reduce(&graph, cliffords.get_data());

            let cooling_config = CoolingConfiguration {
                betas: vec![0.1, 0.5, 10.0],
                num_steps_per_beta: vec![5, 5, 5],
            };

            let (best_graph, best_cliffords, artifacts) =
                search(graph, cliffords, cooling_config, test_cost_function, None);

            let reduced_after =
                stabiliser_group::reduce(&best_graph, best_cliffords.get_data());
            assert_eq!(reduced_init, reduced_after);
            assert!(artifacts.best_cost <= artifacts.initial_cost);
        });
    }
}
