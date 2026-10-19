#!/usr/bin/env python

import ctypes
from ctypes import Structure, POINTER, c_uint64, c_size_t, c_void_p, c_bool
import numpy as np
from math import isnan
from itertools import chain
from lcmh import core_lib
from cabaliser.widget import Widget
from cabaliser.structs import WidgetType
from cabaliser import gates
from cabaliser.operation_sequence import OperationSequence
from cabaliser import local_simulator


def some_widget() -> Widget:
    def some_gate(a, b, c):
        return [
            (gates.H, (c,)),
            (gates.CNOT, (a, b)),
            (gates.H, (a,)),
            (gates.CNOT, (a, b)),
            (gates.H, (b,)),
            (gates.CNOT, (b, c)),
            (gates.R, (a,)),
        ]

    gate = some_gate(0, 1, 2)
    # wid = Widget(n_qubits, max_qubits, teleport_input=False)
    wid = Widget(3, 10)
    ops = OperationSequence(len(gate))
    for opcode, args in gate:
        ops.append(opcode, *args)
    wid(ops)
    wid.decompose()
    return wid


# def buffer_widget(n_qubits, max_qubits) -> Widget:
#     wid = Widget(n_qubits, max_qubits, n_inputs=0)
#     wid.decompose()
#     return wid


def buffer_widget(n_qubits, max_qubits) -> Widget:
    wid = some_widget()
    return wid


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


def get_tableau_from_widget(widget: Widget):
    return ctypes.cast(widget.widget.contents.__tableau, POINTER(Tableau))


class WidgetAlias:
    def __init__(self, widget: Widget):
        self.widget = widget
        self.tableau = get_tableau_from_widget(widget)
        self.queue = widget.widget.contents.queue

    def update(self, widget: WidgetPtrCopy):
        self.tableau.contents.slices_z = widget.slices
        self.queue.contents.cliffords = widget.cliffords
        self.widget.local_cliffords = None


class WidgetPtrCopy:
    def __init__(self, widget: WidgetAlias):
        self.slices = ctypes.pointer(widget.tableau.contents.slices_z.contents)
        self.cliffords = ctypes.pointer(widget.queue.contents.cliffords.contents)

EPS = 1e-6

def main():
    input_widget = some_widget()
    n_qubits = input_widget.get_n_qubits()
    max_qubits = input_widget.get_max_qubits()
    output_widget = buffer_widget(n_qubits, max_qubits)

    input_alias = WidgetAlias(input_widget)
    output_alias = WidgetAlias(output_widget)
    input_copy = WidgetPtrCopy(input_alias)
    output_copy = WidgetPtrCopy(output_alias)

    artifacts = core_lib.core_lib.lcmh_search_with_defaults(
        n_qubits,
        input_copy.slices,
        input_copy.cliffords,
        output_copy.slices,
        output_copy.cliffords,
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

    for i in range(1):
        print(i)
        input_state = local_simulator.kr(
            # local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.state_prep(*list(np.random.random(2))),
            local_simulator.plus_state,
            # local_simulator.state_prep(*list(np.random.random(2))),
            local_simulator.plus_state,
            local_simulator.plus_state,
            # local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.state_prep(*list(np.random.random(2))),
        )

        input_widget_state = local_simulator.simulate_widget(
            input_widget, input_state=input_state, trace_graph=True
        )
        output_widget_state = local_simulator.simulate_widget(
            output_widget, input_state=input_state, trace_graph=True
        )

        # input_alias.update(output_copy)
        # # input_widget.tableau_print()
        # # print(input_widget.get_local_cliffords().to_list())
        # output_widget_state = local_simulator.simulate_widget(
        #     input_widget, input_state=input_state, trace_graph=True
        # )
        # input_alias.update(input_copy)
        # # input_widget.tableau_print()
        # # print(input_widget.get_local_cliffords().to_list())

        dephase(input_widget_state)
        dephase(output_widget_state)

        if (np.abs(input_widget_state - output_widget_state) > EPS).any():
            # print("Operation: ", ops, len(operation))
            print(input_widget_state)
            print(output_widget_state)
            # print(widget_state)
            assert False


def dephase(vector: np.array) -> bool:
    try:
        val = next(filter(lambda x: np.abs(x) > EPS, chain(iter(vector))))
        if isnan(val[0].real):
            return False
        vector /= val
        return True
    except StopIteration:  # Whole vector empty
        return False


if __name__ == "__main__":
    main()
