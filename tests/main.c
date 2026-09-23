#include <stddef.h>
#include <stdint.h>

#include "lcmh.h"

double cost_function(uint32_t _, uint32_t num_edges, uint32_t max_degree,
                     uint32_t num_executed_lcs) {
  return (double)(num_edges + max_degree + num_executed_lcs);
}

int main(void) {
  size_t n_qubits = 1;
  uint64_t slices_mem = 0; // enough for one qubit
  uint64_t *slice = &slices_mem;
  uint64_t **slices = &slice;
  uint64_t num_steps = 5;
  uint64_t output_slices_mem = 0;
  uint64_t *output_slice = &output_slices_mem;
  uint64_t **output_slices = &output_slice;
  direct_search(n_qubits, slices, num_steps, cost_function, output_slices);
}
