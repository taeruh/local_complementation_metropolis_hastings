#!/usr/bin/env python

from lcmh import core_lib
from cabaliser.widget import Widget
from cabaliser import gates
from cabaliser.operation_sequence import OperationSequence


# copied out of a cabaliser test
def toffoli_widget() -> Widget:
    _I_ = 0
    _T_ = 1
    _Tdag_ = 2
    n_qubits = 3
    max_qubits = 20

    def toffoli_gate(ctrl_a, ctrl_b, targ):
        return [
            (gates.RZ, (ctrl_a, _T_)),
            (gates.RZ, (ctrl_b, _T_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.H, (targ,)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (ctrl_b, _Tdag_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.H, (targ,)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
        ]

    toffoli = toffoli_gate(0, 1, 2)
    wid = Widget(n_qubits, max_qubits)  # , teleport_input=False)
    ops = OperationSequence(len(toffoli))
    for opcode, args in toffoli:
        ops.append(opcode, *args)
    wid(ops)
    wid.decompose()
    return wid


def buffer_widget(n_qubits, max_qubits) -> Widget:
    wid = Widget(n_qubits, max_qubits, n_inputs=0)
    # print(wid.get_local_cliffords().to_list())
    # wid.tableau_print(); print()
    wid.decompose()
    return wid


import ctypes
from ctypes import Structure, POINTER, c_uint64, c_size_t, c_void_p, c_bool


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


def get_slices(widget):
    tableau_ptr = ctypes.cast(widget.widget.contents.__tableau, POINTER(Tableau))
    return tableau_ptr.contents.slices_z  # I thought it was on the x slices ...?


def get_cliffords(widget):
    return widget.widget.contents.queue.contents.cliffords


if __name__ == "__main__":
    input_widget = toffoli_widget()
    effective_n_qubits = input_widget.get_n_qubits()
    n_qubits = input_widget.get_n_qubits()
    max_qubits = input_widget.get_max_qubits()
    output_widget = buffer_widget(n_qubits, max_qubits)

    input_slices = get_slices(input_widget)
    output_slices = get_slices(output_widget)
    input_cliffords = get_cliffords(input_widget)
    output_cliffords = get_cliffords(output_widget)

    print(input_widget.get_local_cliffords().to_list())

    # # core_lib.core_lib.lcmh_search_with_defaults_no_return(
    # artifacts = core_lib.core_lib.lcmh_search_with_defaults(
    #     n_qubits, input_slices, input_cliffords, output_slices, output_cliffords
    # )
    artifacts = core_lib.core_lib.lcmh_search_with_defaults_except_cooling_config(
        n_qubits,
        input_slices,
        input_cliffords,
        output_slices,
        output_cliffords,
        core_lib.LcmhCoolingConfiguration([2.0, 100.0], [3, 3]),
    )

    print(artifacts.initial_cost)
    print(artifacts.final_cost)
    print(artifacts.get_costs())

    input_widget.tableau_print()
    print()
    output_widget.tableau_print()
    print()
    output_widget.local_cliffords = None
    print(input_widget.get_local_cliffords().to_list())
    print(output_widget.get_local_cliffords().to_list())
