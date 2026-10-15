use rand::{RngExt, SeedableRng, distr::StandardUniform, rngs::SysRng};
use rand_pcg::Pcg64Mcg;

use crate::{
    c_interface::LcmhCostFunction,
    graph::{Graph, HashSet},
    one_local_clifford::Clifford,
};

#[derive(Debug)]
pub struct CoolingConfiguration {
    pub betas: Vec<f64>,
    pub num_steps_per_beta: Vec<usize>,
}

// TODO: make this here generic so that we don't have to copy when it comes from C
#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
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

    pub fn into_data(self) -> Vec<Clifford> {
        self.cliffords
    }

    pub fn local_complementation(&mut self, node: usize, neighbours: &HashSet<usize>) {
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

pub fn search(
    graph: Graph,
    mut cliffords: Cliffords,
    cooling_config: CoolingConfiguration,
    cost_function: LcmhCostFunction,
    seed: Option<u64>,
) -> (Graph, Cliffords, SearchArtifacts) {
    // cliffords.cliffords.iter_mut().for_each(|c| {
    //     c.invert();
    // });
    let mut search = Search::new(graph, cliffords, cooling_config, cost_function, seed);
    search.search();
    // search.best_cliffords.cliffords.iter_mut().for_each(|c| {
    //     c.invert();
    // });
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
        let rng = match seed {
            Some(seed) => Pcg64Mcg::seed_from_u64(seed),
            None => Pcg64Mcg::try_from_rng(&mut SysRng)
                .expect("Failed to create random number generator from entropy"),
        };
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
            let neighbours = self.graph.get_neighbours(node).unwrap();
            let new_cost = (self.cost_function)(&self.graph);
            let delta_cost = new_cost - current_cost;
            if delta_cost < 0.0
                || self.rng.sample::<f64, StandardUniform>(StandardUniform)
                    < (-beta * delta_cost).exp()
            {
                current_cost = new_cost;
                self.artifacts.costs.push(current_cost);
                self.cliffords.local_complementation(node, neighbours);
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

    extern "C" fn test_cost_function(graph: &Graph) -> f64 {
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

        let cliffords = Cliffords {
            cliffords: Vec::from_iter((0..graph.num_nodes()).map(|_| Clifford::I)),
        };

        // println!("{:?}", graph);

        let cooling_config = CoolingConfiguration {
            betas: vec![0.1, 0.5, 10.0],
            num_steps_per_beta: vec![5, 5, 5],
        };

        let _ = search(graph, cliffords, cooling_config, test_cost_function, None);
    }
}
