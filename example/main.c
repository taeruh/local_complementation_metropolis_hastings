#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "lcmh.h"

#define CHUNK_OBJ uint64_t
#define CHUNK_SIZE sizeof(CHUNK_OBJ)
#define NUM_CHUNKS_PER_ROW(n_qubits) ((n_qubits + CHUNK_SIZE - 1) / CHUNK_SIZE)

struct PseudoCabaliserGraph {
  size_t n_qubits;
  uint64_t **slices;
};
typedef struct PseudoCabaliserGraph PseudoCabaliserGraph;

void flush_slices(size_t n_qubits, uint64_t *slices_mem) {
  size_t num_chunks = NUM_CHUNKS_PER_ROW(n_qubits) * n_qubits;
  for (size_t i = 0; i < num_chunks; i++) {
    slices_mem[i] = 0;
  }
}

PseudoCabaliserGraph create_pseudo_cabaliser_graph(size_t n_qubits) {
  size_t num_chunks_per_row = NUM_CHUNKS_PER_ROW(n_qubits);
  uint64_t *slices_mem = malloc(n_qubits * num_chunks_per_row * CHUNK_SIZE);
  uint64_t **slices = malloc(n_qubits * sizeof(size_t));
  for (size_t i = 0; i < n_qubits; i++) {
    slices[i] = slices_mem + i * num_chunks_per_row;
  }
  flush_slices(n_qubits, slices[0]);
  return (PseudoCabaliserGraph){.n_qubits = n_qubits, .slices = slices};
}

void free_pseudo_cabaliser_graph(PseudoCabaliserGraph graph) {
  free(graph.slices[0]);
  free(graph.slices);
}

void print_adj_matrix(size_t n_qubits, uint64_t **slices) {
  for (size_t i = 0; i < n_qubits; i++) {
    for (size_t j = 0; j < n_qubits; j++) {
      if (slices[i][0] & (1ull << j)) {
        printf("1 ");
      } else {
        printf("0 ");
      }
    }
    printf("\n");
  }
}

double cost_function(const LcmhGraph *graph, size_t num_ops) {
  return (double)(lcmh_get_num_edges(graph)) + (double)(num_ops) * 0.01;
}

int main(void) {
  size_t n_qubits = 5;

  // assume the following is actually a graph coming from cabaliser, i.e.,
  // tableau_t.n_qubits and tableau_t.slices_x {{
  PseudoCabaliserGraph input_graph = create_pseudo_cabaliser_graph(5);
  input_graph.slices[0][0] |= 1ull << 1;
  input_graph.slices[1][0] |= 1ull << 0;
  input_graph.slices[0][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 0;
  input_graph.slices[1][0] |= 1ull << 2;
  input_graph.slices[2][0] |= 1ull << 1;
  input_graph.slices[1][0] |= 1ull << 3;
  input_graph.slices[3][0] |= 1ull << 1;
  input_graph.slices[2][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 2;
  input_graph.slices[3][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 3;
  // }}

  printf("Initial adjacency matrix:\n");
  print_adj_matrix(n_qubits, input_graph.slices);

  // a cooling configuration (note that the scale of the beta values depends on
  // the scale of the cost function) size_t num_betas = 3; double betas[] =
  // {0.1, 0.5, 1.0}; size_t num_steps_per_beta[] = {2, 2, 2};
  LcmhCoolingConfiguration cooling_config = {.num_betas = 3,
                                             .betas = (double[]){0.1, 0.5, 1.0},
                                             .num_steps_per_beta =
                                                 (size_t[]){2, 2, 2}};

  // not seeding randomly for reproducibility here; if you want to seed
  // randomly, set `seed_from_entropy` to 1
  uint8_t seed_from_entropy = 0;
  uint64_t seed = 0;

  // allocate memory slices which will hold the transformed adjacency matrix
  PseudoCabaliserGraph output_graph = create_pseudo_cabaliser_graph(n_qubits);

  // run the search
  LcmhSearchArtifacts artifacts = lcmh_direct_search(
      n_qubits, input_graph.slices, cooling_config, &cost_function,
      seed_from_entropy, seed, output_graph.slices);

  printf("\nOutput adjacency matrix:\n");
  print_adj_matrix(n_qubits, output_graph.slices);

  printf("\nCost at each (accepted) MH step:\n");
  for (size_t i = 0; i < artifacts.length_costs; i++) {
    printf("Step %zu: %f\n", i, artifacts.costs[i]);
  }

  lcmh_free_search_artifacts(artifacts);

  // apparently, the results where not great (the final cost is higher than the
  // initial cost); this is because the betas where to high
  //
  // in general, one may want to do the search multiple times with different
  // configs and then pick the best result
  //
  // the `lcmh_direct_search` function internally creates a new representation
  // (LcmhGraph) of the graph based on hashsets; one may want to store this
  // representation and reuse it for multiple searches to avoid the overhead of
  // doing this transformation multiple times

  printf("\nNew try\n");

  LcmhGraph *lcmh_graph = lcmh_transform_cabaliser_graph_to_lcmh_graph(
      n_qubits, input_graph.slices);

  // first try

  seed = 5;
  cooling_config.betas = (double[]){0.1, 1.0, 1.0};
  cooling_config.num_steps_per_beta = (size_t[]){2, 2, 2};
  LcmhGraph *lcmh_graph_clone = lcmh_clone_lcmh_graph(lcmh_graph);
  artifacts = lcmh_search(lcmh_graph_clone, cooling_config, &cost_function,
                          seed_from_entropy, seed);
  printf("Final cost first try: %f\n",
         artifacts.costs[artifacts.length_costs - 1]);

  // better, but we can probably improve on it

  lcmh_free_search_artifacts(artifacts);
  lcmh_free_lcmh_graph(lcmh_graph_clone);

  // let's try again

  cooling_config.num_betas = 4;
  cooling_config.betas = (double[]){0.1, 1.0, 10.0, 50.0};
  cooling_config.num_steps_per_beta = (size_t[]){3, 3, 3, 3};
  artifacts = lcmh_search(lcmh_graph, cooling_config, &cost_function,
                          seed_from_entropy, seed);
  printf("Final cost second try: %f\n",
         artifacts.costs[artifacts.length_costs - 1]);

  // this is better; let's pick this is the final result

  flush_slices(n_qubits, output_graph.slices[0]);
  lcmh_transform_lcmh_graph_to_cabaliser_graph(lcmh_graph, output_graph.slices);
  lcmh_free_lcmh_graph(lcmh_graph);

  printf("\nFinal adjacency matrix:\n");
  print_adj_matrix(n_qubits, output_graph.slices);

  // let's view the lc operations that were applied to the graph (encoded
  // according to instructions_table.h in cabaliser); only view the first 10...
  size_t num_ops_to_view = artifacts.length_local_clifford_ops < 10
                               ? artifacts.length_local_clifford_ops
                               : 10;

  printf("\nLocal Clifford operations applied (first %zu):\n", num_ops_to_view);
  for (size_t i = 0; i < num_ops_to_view; i++) {
    LcmhSingleQubitCliffordOperation op = artifacts.local_clifford_ops[i];
    printf("Local Clifford operation: node %zu, operation %u\n", op.node,
           op.operation);
  }

  lcmh_free_search_artifacts(artifacts);

  free_pseudo_cabaliser_graph(input_graph);
  free_pseudo_cabaliser_graph(output_graph);
}
