import ctypes
from ctypes import (
    Structure,
    POINTER,
    c_void_p,
    c_size_t,
    c_uint64,
    c_bool,
    c_byte,
)

class CliffordBuffer:
    def __init__(self, n_qubits):
        self.n_qubits = n_qubits
        cliffords = [(1 << 5) for _ in range(n_qubits)]
        self.cliffords = (c_byte * n_qubits)(*cliffords)

class _Tableau(Structure):
    _fields_ = [
        ("n_qubits", c_size_t),
        ("slice_len", c_size_t),
        ("chunks", c_void_p),
        ("slices_x", POINTER(POINTER(c_uint64))),
        ("slices_z", POINTER(POINTER(c_uint64))),
        ("phases", POINTER(c_uint64)),
        ("orientation", c_bool),
    ]

def get_slices(widget):
    tableau = ctypes.cast(widget.widget.contents.__tableau, POINTER(_Tableau))
    return tableau.contents.slices_z
