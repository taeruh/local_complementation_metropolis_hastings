import ctypes
from ctypes import (
    Structure,
    POINTER,
    c_void_p,
    c_size_t,
    c_uint64,
    c_bool,
)

from cabaliser.widget import Widget


class Tableau(Structure):
    _fields_ = [
        ("n_qubits", c_size_t),
        ("slice_len", c_size_t),
        ("chunks", c_void_p),
        ("slices_x", POINTER(POINTER(c_uint64))),
        ("slices_z", POINTER(POINTER(c_uint64))),
        ("phases", POINTER(c_uint64)),
        ("orientation", c_bool),
    ]


def _get_tableau_from_widget(widget: Widget):
    return ctypes.cast(widget.widget.contents.__tableau, POINTER(Tableau))


class WidgetAlias:
    def __init__(self, widget: Widget):
        self.widget = widget
        self.tableau = _get_tableau_from_widget(widget)
        self.queue = widget.widget.contents.queue

    def update(self, widget: WidgetPtrCopy):
        self.tableau.contents.slices_z = widget.slices
        self.queue.contents.cliffords = widget.cliffords
        self.widget.local_cliffords = None


class WidgetPtrCopy:
    def __init__(self, widget: WidgetAlias):
        self.slices = ctypes.pointer(widget.tableau.contents.slices_z.contents)
        self.cliffords = ctypes.pointer(widget.queue.contents.cliffords.contents)
