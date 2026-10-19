#![deny(unsafe_op_in_unsafe_fn)]
// #![warn(missing_docs)]

use rand::{SeedableRng, TryRng, rngs::SysRng};
use rand_pcg::Pcg64Mcg;

pub mod c_interface;
mod graph;
mod one_local_clifford;
mod search;
#[cfg(test)]
mod stabiliser_group;

fn create_rng(seed: Option<u64>) -> (Pcg64Mcg, u64) {
    let seed = match seed {
        Some(seed) => seed,
        None => SysRng.try_next_u64().expect("Failed to generate random seed"),
    };
    (Pcg64Mcg::seed_from_u64(seed), seed)
}

#[cfg(test)]
mod tests {
    use std::panic;

    use rand::{RngExt, distr::StandardUniform};

    use super::*;
    use crate::{
        graph::Graph, one_local_clifford::Clifford, search::Cliffords, stabiliser_group,
    };

    pub fn create_random_graph(
        rng: &mut Pcg64Mcg,
        max_num_nodes: usize,
    ) -> (Graph, Cliffords) {
        let num_nodes = rng.random_range(0..max_num_nodes);
        let density = rng.random_range(0.0..1.0);
        let mut graph = Graph::new(num_nodes);
        for a in 0..num_nodes {
            for b in (a + 1)..num_nodes {
                if rng.sample::<f64, StandardUniform>(StandardUniform) < density {
                    graph.add_edge(a, b);
                }
            }
        }
        (
            graph,
            Cliffords::new(Vec::from_iter(
                (0..num_nodes)
                    .map(|_| Clifford::from_discriminant(rng.random_range(0..24))),
            )),
        )
    }

    pub fn catch_seed_on_panic(
        seed: Option<u64>,
        f: impl FnOnce(Pcg64Mcg) + panic::UnwindSafe,
    ) {
        let (rng, seed) = create_rng(seed);
        let panic_result = panic::catch_unwind(move || f(rng));
        if let Err(err) = panic_result {
            println!("Seed: {}", seed);
            panic::resume_unwind(err);
        }
    }

    #[test]
    fn local_complementation() {
        catch_seed_on_panic(None, |mut rng| {
            let (mut graph, mut cliffords) = create_random_graph(&mut rng, 10);

            let reduced_init = stabiliser_group::reduce(&graph, cliffords.get_data());

            for node in 0..graph.num_nodes() {
                cliffords.inverse_local_complementation(
                    node,
                    graph.get_neighbours(node).unwrap(),
                );
                graph.local_complementation(node);
                let reduced_after =
                    stabiliser_group::reduce(&graph, cliffords.get_data());
                assert_eq!(reduced_init, reduced_after);
            }
        });
    }
}
