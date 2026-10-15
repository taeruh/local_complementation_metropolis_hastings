// The documentation is missing here for now; however, if you run `cargo doc` in
// the lcmh directory and open `lcmh/target/doc/lcmh/c_interface/index.html` in
// your browser, you'll find the according documentation there. However, with
// Rust syntax instead of C syntax; the pointers translate as follows:
// - `const T *`: `&T` or `*const T`
// - `T *`: `Box<T>`, `&mut T` or `*mut T`
// the integer types as:
// - `size_t`: `usize`
// - `uint<N>_t`: `u<N>`
// and the floating point types as:
// - `double`: `f64`

#include <stddef.h>
#include <stdint.h>

typedef struct LcmhGraph LcmhGraph;

size_t lcmh_get_num_edges(const LcmhGraph *graph);

size_t lcmh_get_max_degree(const LcmhGraph *graph);

size_t lcmh_get_nodes_num_neighbours(const LcmhGraph *graph, size_t node);

struct LcmhCoolingConfiguration {
  size_t num_betas;
  double const *betas;
  size_t const *num_steps_per_beta;
};
typedef struct LcmhCoolingConfiguration LcmhCoolingConfiguration;

typedef uint8_t LcmhSingleQubitClifford;

struct LcmhSearchArtifacts {
  double *costs;
  size_t length_costs;
  double initial_cost;
  double final_cost;
};
typedef struct LcmhSearchArtifacts LcmhSearchArtifacts;

void lcmh_free_search_artifacts(LcmhSearchArtifacts artifacts);

typedef double (*LcmhCostFunction)(const LcmhGraph *graph);

LcmhSearchArtifacts lcmh_search(size_t n_qubits, uint64_t **slices,
                                LcmhSingleQubitClifford *cliffords,
                                uint64_t **output_slices,
                                LcmhSingleQubitClifford *output_cliffords,
                                LcmhCoolingConfiguration cooling_config,
                                LcmhCostFunction cost_function,
                                int seed_from_entropy, uint64_t seed);

void lcmh_search_no_artifacts(size_t n_qubits, uint64_t **slices,
                              LcmhSingleQubitClifford *cliffords,
                              uint64_t **output_slices,
                              LcmhSingleQubitClifford *output_cliffords,
                              LcmhCoolingConfiguration cooling_config,
                              LcmhCostFunction cost_function,
                              int seed_from_entropy, uint64_t seed);

LcmhSearchArtifacts lcmh_search_with_defaults(
    size_t n_qubits, uint64_t **slices, LcmhSingleQubitClifford *cliffords,
    uint64_t **output_slices, LcmhSingleQubitClifford *output_cliffords);

void lcmh_search_with_defaults_no_return(
    size_t n_qubits, uint64_t **slices, LcmhSingleQubitClifford *cliffords,
    uint64_t **output_slices, LcmhSingleQubitClifford *output_cliffords);

LcmhSearchArtifacts lcmh_search_with_defaults_except_cooling_config(
    size_t n_qubits, uint64_t **slices, LcmhSingleQubitClifford *cliffords,
    uint64_t **output_slices, LcmhSingleQubitClifford *output_cliffords,
    LcmhCoolingConfiguration cooling_config);

void lcmh_search_with_defaults_except_cooling_config_no_return(
    size_t n_qubits, uint64_t **slices, LcmhSingleQubitClifford *cliffords,
    uint64_t **output_slices, LcmhSingleQubitClifford *output_cliffords,
    LcmhCoolingConfiguration cooling_config);
