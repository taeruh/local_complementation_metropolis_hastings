// The documentation is missing here for now; however, if you run `cargo doc` in
// the lcmh directory and open `lcmh/target/doc/lcmh/interface/index.html` in
// your browser, you'll find the according documentation there.

#include <stddef.h>
#include <stdint.h>

typedef struct LcmhGraph LcmhGraph;

LcmhGraph *lcmh_transform_cabaliser_graph_to_lcmh_graph(size_t n_qubits,
                                                        uint64_t **slices);

size_t lcmh_transform_lcmh_graph_to_cabaliser_graph(const LcmhGraph *graph,
                                                    uint64_t **slices_buffer);

LcmhGraph *lcmh_clone_lcmh_graph(const LcmhGraph *graph);

void lcmh_free_lcmh_graph(LcmhGraph *graph);

size_t lcmh_get_num_edges(const LcmhGraph *graph);

size_t lcmh_get_max_degree(const LcmhGraph *graph);

size_t lcmh_get_nodes_num_neighbours(const LcmhGraph *graph, size_t node);

struct LcmhCoolingConfiguration {
  size_t num_betas;
  double *betas;
  size_t *num_steps_per_beta;
};
typedef struct LcmhCoolingConfiguration LcmhCoolingConfiguration;

typedef uint8_t LcmhSingleQubitClifford;

struct LcmhSingleQubitCliffordOperation {
  LcmhSingleQubitClifford operation;
  size_t node;
};
typedef struct LcmhSingleQubitCliffordOperation
    LcmhSingleQubitCliffordOperation;

struct LcmhSearchArtifacts {
  LcmhSingleQubitCliffordOperation *local_clifford_ops;
  size_t length_lc_ops;
  double *costs;
  size_t length_costs;
};
typedef struct LcmhSearchArtifacts LcmhSearchArtifacts;

void lcmh_free_search_artifacts(LcmhSearchArtifacts artifacts);

typedef double (*CostFunction)(const LcmhGraph *graph, size_t num_executed_lcs);

LcmhSearchArtifacts lcmh_search(LcmhGraph *graph,
                                LcmhCoolingConfiguration cooling_config,
                                CostFunction cost_function,
                                int seed_from_entropy, uint64_t seed);

LcmhSearchArtifacts lcmh_direct_search(size_t n_qubits, uint64_t **slices,
                                       LcmhCoolingConfiguration cooling_config,
                                       CostFunction cost_function,
                                       int seed_from_entropy, uint64_t seed,
                                       uint64_t **output_slices);
