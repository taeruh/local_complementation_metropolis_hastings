from ctypes import (
    POINTER,
    c_size_t,
    c_uint64,
    c_byte,
)

import os
from ctypes import cdll

core_lib = cdll.LoadLibrary(f"{os.path.dirname(__file__)}/libs/liblcmh.so")

core_lib.lcmh_simpler_direct_search.argtypes = [
    c_size_t,
    POINTER(POINTER(c_size_t)),
    POINTER(POINTER(c_size_t)),
    POINTER(c_byte),
]
