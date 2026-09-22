#include <stdint.h>

void search(uint32_t num_steps,
            double (*cost_function)(uint32_t num_vertices, uint32_t num_edges,
                                    uint32_t max_degree,
                                    uint32_t num_executed_lcs));
