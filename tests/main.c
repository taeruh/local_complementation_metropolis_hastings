#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "lcmh.h"

#define CHUNK_OBJ uint64_t
#define CHUNK_SIZE sizeof(CHUNK_OBJ)

double cost_function(LcmhGraph *graph, size_t num_ops) {
  return (double)(lcmh_get_num_edges(graph) + num_ops);
}

int main(void) {
  size_t n_qubits = 3;
  size_t num_chunks_per_row = (n_qubits + CHUNK_SIZE - 1) / CHUNK_SIZE;
  uint64_t *slices_mem = malloc(n_qubits * num_chunks_per_row * CHUNK_SIZE);
  uint64_t **slices = malloc(n_qubits * sizeof(size_t));
  for (size_t i = 0; i < n_qubits; i++) {
    slices[i] = slices_mem + i * num_chunks_per_row;
  }
  slices[0][0] = 1ull << 1;
  slices[1][0] = 1ull << 0;

  size_t num_betas = 1;
  double *betas = malloc(num_betas * sizeof(double));
  betas[0] = 1.0;
  size_t *num_steps_per_beta = malloc(num_betas * sizeof(size_t));
  num_steps_per_beta[0] = 5;
  LcmhCoolingConfiguration cooling_config = {
      .num_betas = num_betas,
      .betas = betas,
      .num_steps_per_beta = num_steps_per_beta,
  };

  uint8_t seed_from_entropy = 1;
  uint64_t seed = 0;

  uint64_t *output_slices_mem =
      malloc(n_qubits * num_chunks_per_row * CHUNK_SIZE);
  uint64_t **output_slices = malloc(n_qubits * sizeof(size_t));
  for (size_t i = 0; i < n_qubits; i++) {
    output_slices[i] = output_slices_mem + i * num_chunks_per_row;
  }

  LcmhSearchArtifacts artifacts =
      lcmh_direct_search(n_qubits, slices, cooling_config, &cost_function,
                         seed_from_entropy, seed, output_slices);
}
