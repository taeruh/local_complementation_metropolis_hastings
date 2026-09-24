#include <stddef.h>
#include <stdint.h>

// The documentation is missing here for now; however, if you run `cargo doc` in
// the lcmh directory and open `lcmh/target/doc/lcmh/interface/index.html` in
// your browser, you'll find the according documentation there.

typedef struct LcmhGraph LcmhGraph;

LcmhGraph *lcmh_transform_cabaliser_graph_to_lcmh_graph(size_t n_qubits,
                                                        uint64_t **slices);

size_t lcmh_transform_lcmh_graph_to_cabaliser_graph(LcmhGraph *graph,
                                                    uint64_t **slices_buffer);

LcmhGraph *lcmh_clone_lcmh_graph(LcmhGraph *graph);

void lcmh_free_lcmh_graph(LcmhGraph *graph);

size_t lcmh_get_num_edges(LcmhGraph *graph);

size_t lcmh_get_max_degree(LcmhGraph *graph);

size_t lcmh_get_nodes_num_neighbours(LcmhGraph *graph, size_t node);

struct LcmhCoolingConfiguration {
  size_t num_betas;
  double *betas;
  size_t *num_steps_per_beta;
};
typedef struct LcmhCoolingConfiguration LcmhCoolingConfiguration;

struct LcmhLocalComplementationCliffords {
  uint8_t *ops;
  size_t length;
};
typedef struct LcmhLocalComplementationCliffords
    LcmhLocalComplementationCliffords;

struct LcmhSearchArtifacts {
  struct LcmhLocalComplementationCliffords lc_ops;
  double cost;
};
typedef struct LcmhSearchArtifacts LcmhSearchArtifacts;

void lcmh_free_local_complementation_cliffords(
    struct LcmhLocalComplementationCliffords *lc_ops);

LcmhSearchArtifacts lcmh_search(
    LcmhGraph *graph, LcmhCoolingConfiguration cooling_config,
    double (*cost_function)(LcmhGraph *graph, size_t num_executed_lcs),
    int seed_from_entropy, uint64_t seed);

LcmhSearchArtifacts lcmh_direct_search(
    size_t n_qubits, uint64_t **slices, LcmhCoolingConfiguration cooling_config,
    double (*cost_function)(LcmhGraph *graph, size_t num_executed_lcs),
    int seed_from_entropy, uint64_t seed, uint64_t **output_slices);
