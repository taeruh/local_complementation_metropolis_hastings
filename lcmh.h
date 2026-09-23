#include <stddef.h>
#include <stdint.h>

struct SearchArtifacts {
  // ...
};
typedef struct SearchArtifacts SearchArtifacts;

SearchArtifacts direct_search(
    size_t n_qubits, uint64_t **slices, uint32_t num_steps,
    double (*cost_function)(uint32_t num_vertices, uint32_t num_edges,
                            uint32_t max_degree, uint32_t num_executed_lcs),
    uint64_t **output_slices);
