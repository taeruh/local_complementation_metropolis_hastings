#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "lcmh.h"

#define CHUNK_OBJ uint64_t
#define CHUNK_SIZE sizeof(CHUNK_OBJ)
#define NUM_CHUNKS_PER_COLUMN(n_qubits)                                        \
  ((n_qubits + CHUNK_SIZE - 1) / CHUNK_SIZE)

// this is basically what we get from cabaliser
struct PseudoCabaliserGraph {
  size_t n_qubits;
  uint64_t **slices;  // adjacency matrix
  uint8_t *cliffords; // local clifford operations applied to each qubit
};
typedef struct PseudoCabaliserGraph PseudoCabaliserGraph;

// set everything to zero
void flush_slices(size_t n_qubits, uint64_t *slices_mem) {
  size_t num_chunks = NUM_CHUNKS_PER_COLUMN(n_qubits) * n_qubits;
  for (size_t i = 0; i < num_chunks; i++) {
    slices_mem[i] = 0;
  }
}

// initialise a pseudo cabaliser graph initialised to zero (i.e., no edges)
PseudoCabaliserGraph create_pseudo_cabaliser_graph(size_t n_qubits) {
  size_t num_chunks_per_column = NUM_CHUNKS_PER_COLUMN(n_qubits);
  uint64_t *slices_mem = malloc(n_qubits * num_chunks_per_column * CHUNK_SIZE);
  uint64_t **slices = malloc(n_qubits * sizeof(size_t));
  uint8_t *cliffords = malloc(n_qubits * sizeof(uint8_t));
  for (size_t i = 0; i < n_qubits; i++) {
    slices[i] = slices_mem + i * num_chunks_per_column;
  }
  size_t num_chunks = NUM_CHUNKS_PER_COLUMN(n_qubits) * n_qubits;
  for (size_t i = 0; i < num_chunks; i++) {
    slices_mem[i] = 0;
  }
  for (size_t i = 0; i < n_qubits; i++) {
    cliffords[i] = 1 << 5;
  }
  return (PseudoCabaliserGraph){
      .n_qubits = n_qubits, .slices = slices, .cliffords = cliffords};
}

void free_pseudo_cabaliser_graph(PseudoCabaliserGraph graph) {
  free(graph.slices[0]);
  free(graph.slices);
  free(graph.cliffords);
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

void print_cliffords(size_t n_qubits, uint8_t *cliffords) {
  for (size_t i = 0; i < n_qubits; i++) {
    printf("node %zu: operation %u\n", i, cliffords[i]);
  }
}

// this here will be the most important bit; the cost function that decides how
// good a graph is (the lower the cost, the better); this here is just a simple
// example where we only care about the total number of edges in the graph (the
// fewer edges, the better)
double cost_function(const LcmhGraph *graph) {
  // printf("edges: %zu, %f\n", lcmh_get_num_edges(graph),
  //        (double)lcmh_get_num_edges(graph));
  return (double)(lcmh_get_num_edges(graph));
}

int main(void) {
  size_t n_qubits = 5;

  // assume the following is actually a graph coming from cabaliser, i.e.,
  // tableau_t.n_qubits and tableau_t.slices_x {{
  PseudoCabaliserGraph input_graph = create_pseudo_cabaliser_graph(5);
  // create some edges (there is a better way to do this):
  // edge (0, 1)
  input_graph.slices[0][0] |= 1ull << 1;
  input_graph.slices[1][0] |= 1ull << 0;
  // edge (0, 4)
  input_graph.slices[0][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 0;
  // edge (1, 2)
  input_graph.slices[1][0] |= 1ull << 2;
  input_graph.slices[2][0] |= 1ull << 1;
  // edge (1, 3)
  input_graph.slices[1][0] |= 1ull << 3;
  input_graph.slices[3][0] |= 1ull << 1;
  // edge (2, 4)
  input_graph.slices[2][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 2;
  // edge (3, 4)
  input_graph.slices[3][0] |= 1ull << 4;
  input_graph.slices[4][0] |= 1ull << 3;
  // }}

  printf("Initial adjacency matrix:\n");
  print_adj_matrix(n_qubits, input_graph.slices);

  // a cooling configuration (note that the scale of the beta values depends on
  // the scale of the cost function)
  LcmhCoolingConfiguration cooling_config = {
      .num_betas = 4,
      .betas = (double[]){0.1, 1.0, 10.0, 0.1},
      .num_steps_per_beta = (size_t[]){2, 2, 2, 2, 2}};

  // not seeding randomly for reproducibility here; if you want to seed
  // randomly, set `seed_from_entropy` to 1 (the value of `seed` does not matter
  // then)
  uint8_t seed_from_entropy = 0;
  uint64_t seed = 0;

  // run the search
  LcmhSearchArtifacts artifacts =
      lcmh_search(n_qubits, input_graph.slices, input_graph.cliffords,
                  cooling_config, &cost_function, seed_from_entropy, seed);

  printf("\nOutput adjacency matrix:\n");
  print_adj_matrix(n_qubits, input_graph.slices);

  printf("\nLocal Clifford operations applied:\n");
  print_cliffords(n_qubits, input_graph.cliffords);

  printf("\nInitial cost: %f\n", artifacts.initial_cost);
  printf("Final cost: %f\n", artifacts.best_cost);

  // printf("\nCost at each (accepted) MH step:\n");
  // for (size_t i = 0; i < artifacts.length_costs; i++) {
  //   printf("Step %zu: %f\n", i, artifacts.costs[i]);
  // }

  lcmh_free_search_artifacts(artifacts);
  free_pseudo_cabaliser_graph(input_graph);
}
