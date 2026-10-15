#!/usr/bin/env python

from lcmh import core_lib
from cabaliser.widget import Widget
from cabaliser import gates
from cabaliser.operation_sequence import OperationSequence
from cabaliser import local_simulator
import numpy as np
from math import isnan
from itertools import chain
import ctypes


def dephase(vector: np.array) -> bool:
    try:
        val = next(filter(lambda x: np.abs(x) > EPS, chain(iter(vector))))
        if isnan(val[0].real):
            return False
        vector /= val
        return True
    except StopIteration:  # Whole vector empty
        return False


# copied out of a cabaliser test
def toffoli_widget() -> Widget:
    _I_ = 0
    _T_ = 1
    _Tdag_ = 2
    n_qubits = 3
    max_qubits = 20

    # (gates.RZ, (ctrl_a, _T_)),
    # (gates.CNOT, (ctrl_b, targ)),
    # (gates.H, (targ,)),
    # (gates.CNOT, (ctrl_a, ctrl_b)),

    def toffoli_gate(ctrl_a, ctrl_b, targ):
        return [
            (gates.H, (targ,)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.H, (ctrl_a,)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.H, (ctrl_b,)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.R, (ctrl_a,)),
        ]

    toffoli = toffoli_gate(0, 1, 2)
    # wid = Widget(n_qubits, max_qubits, teleport_input=False)
    wid = Widget(n_qubits, max_qubits)
    ops = OperationSequence(len(toffoli))
    for opcode, args in toffoli:
        ops.append(opcode, *args)
    wid(ops)
    wid.decompose()
    return wid


def buffer_widget(n_qubits, max_qubits) -> Widget:
    wid = Widget(n_qubits, max_qubits, n_inputs=0)
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


def get_tableau(widget):
    return ctypes.cast(widget.widget.contents.__tableau, POINTER(Tableau))


def get_slices(tableau):
    return tableau.contents.slices_z  # I thought it was on the x slices ...?


def get_cliffords(widget):
    return widget.widget.contents.queue.contents.cliffords


if __name__ == "__main__":
    input_widget = toffoli_widget()
    n_qubits = input_widget.get_n_qubits()
    max_qubits = input_widget.get_max_qubits()
    output_widget = buffer_widget(n_qubits, max_qubits)

    input_tableau = get_tableau(input_widget)
    output_tableau = get_tableau(output_widget)
    input_slices = get_slices(input_tableau)
    output_slices = get_slices(output_tableau)
    input_cliffords = get_cliffords(input_widget)
    output_cliffords = get_cliffords(output_widget)

    artifacts = core_lib.core_lib.lcmh_search_with_defaults(
        n_qubits, input_slices, input_cliffords, output_slices, output_cliffords
    )

    print(artifacts.initial_cost)
    print(artifacts.final_cost)

    input_widget.tableau_print()
    print()
    output_widget.tableau_print()
    print()
    output_widget.local_cliffords = None
    print(input_widget.get_local_cliffords().to_list())
    print(output_widget.get_local_cliffords().to_list())
    print()

    adjust_widget = input_widget.json()

    d_input_slices = input_slices.contents
    d_output_slices = output_slices.contents
    d_input_cliffords = input_cliffords.contents
    d_output_cliffords = output_cliffords.contents

    EPS = 1e-6
    for i in range(5):
        print(i)
        input_state = local_simulator.kr(
                local_simulator.state_prep(*list(np.random.random(2))),
                local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.plus_state,
            local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.plus_state,
            # local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.state_prep(*list(np.random.random(2))),
        )

        input_widget_state = local_simulator.simulate_widget(
            input_widget, input_state=input_state, trace_graph=True
        )

        input_tableau.contents.slices_z = ctypes.pointer(d_output_slices)
        input_widget.widget.contents.queue.contents.cliffords = ctypes.pointer(
            d_output_cliffords
        )
        
        # input_widget.local_cliffords = None
        # input_widget.tableau_print()
        # print(input_widget.get_local_cliffords().to_list())

        output_widget_state = local_simulator.simulate_widget(
            input_widget, input_state=input_state, trace_graph=True
        )

        input_tableau.contents.slices_z = ctypes.pointer(d_input_slices)
        input_widget.widget.contents.queue.contents.cliffords = ctypes.pointer(
            d_input_cliffords
        )

        dephase(input_widget_state)
        dephase(output_widget_state)

        if (np.abs(input_widget_state - output_widget_state) > EPS).any():
            # print("Operation: ", ops, len(operation))
            print(input_widget_state)
            print(output_widget_state)
            # print(widget_state)
            assert False
