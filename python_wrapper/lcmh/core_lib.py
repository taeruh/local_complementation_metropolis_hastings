# The documentation is missing here for now; however, if you run `cargo doc` in
# the lcmh directory and open `lcmh/target/doc/lcmh/c_interface/index.html` in
# your browser, you'll find the according documentation there. However, with
# Rust syntax Python syntax...


from ctypes import (
    POINTER,
    c_size_t,
    c_uint64,
    c_byte,
    cdll,
    Structure,
    c_void_p,
    c_double,
)

import os

core_lib = cdll.LoadLibrary(f"{os.path.dirname(__file__)}/libs/liblcmh.so")


class LcmhSearchArtifacts(Structure):
    _fields_ = [
        ("_costs", POINTER(c_double)),
        ("_length_costs", c_size_t),
        ("initial_cost", c_double),
        ("best_cost", c_double),
    ]

    def get_costs(self):
        return [self._costs[i] for i in range(self._length_costs)]

    def __del__(self):
        core_lib.lcmh_free_search_artifacts(self)


class LcmhCoolingConfiguration(Structure):
    _fields_ = [
        ("num_betas", c_size_t),
        ("betas", POINTER(c_double)),
        ("num_steps_per_beta", POINTER(c_size_t)),
    ]

    def __init__(self, betas, num_steps_per_beta):
        assert len(betas) == len(
            num_steps_per_beta
        ), "betas and num_steps_per_beta must have the same length"
        self.num_betas = len(betas)
        self.betas = (c_double * len(betas))(*betas)
        self.num_steps_per_beta = (c_size_t * len(num_steps_per_beta))(
            *num_steps_per_beta
        )


lcmh_search_with_defaults_no_return = core_lib.lcmh_search_with_defaults_no_return
lcmh_search_with_defaults_no_return.argtypes = [
    c_size_t,
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
]

lcmh_search_with_defaults = core_lib.lcmh_search_with_defaults
lcmh_search_with_defaults.argtypes = [
    c_size_t,
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
]
lcmh_search_with_defaults.restype = LcmhSearchArtifacts

lcmh_search_with_defaults_except_cooling_config_no_return = (
    core_lib.lcmh_search_with_defaults_except_cooling_config_no_return
)
lcmh_search_with_defaults_except_cooling_config_no_return.argtypes = [
    c_size_t,
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    LcmhCoolingConfiguration,
]

lcmh_search_with_defaults_except_cooling_config = (
    core_lib.lcmh_search_with_defaults_except_cooling_config
)
lcmh_search_with_defaults_except_cooling_config.argtypes = [
    c_size_t,
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
    LcmhCoolingConfiguration,
]
lcmh_search_with_defaults_except_cooling_config.restype = LcmhSearchArtifacts
