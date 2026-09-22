#include "lcmh.h"
#include <stdint.h>

double cost_function(uint32_t _, uint32_t num_edges, uint32_t max_degree,
                     uint32_t num_executed_lcs) {
  return (double)(num_edges + max_degree + num_executed_lcs);
}

int main(void) {
  uint32_t vertex_selection_type = 3;
  search(vertex_selection_type, cost_function);
}
