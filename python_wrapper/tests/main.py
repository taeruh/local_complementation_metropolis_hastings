#!/usr/bin/env python

import ctypes
from ctypes import POINTER, c_byte
import numpy as np
from math import isnan
from itertools import chain
from lcmh import core_lib
from lcmh.cabaliser_interface_helper import CliffordBuffer, WidgetAlias, WidgetPtrCopy
from cabaliser.widget import Widget
from cabaliser.structs import WidgetType
from cabaliser import gates
from cabaliser.operation_sequence import OperationSequence
from cabaliser.io_array_wrappers import LocalCliffords
from cabaliser import local_simulator


def some_widget() -> Widget:
    def some_gate(a, b, c):
        return [
            (gates.S, (b,)),
            (gates.R, (c,)),
            (gates.H, (c,)),
            (gates.CNOT, (a, b)),
            (gates.H, (a,)),
            (gates.CNOT, (a, b)),
            (gates.H, (b,)),
            (gates.CNOT, (b, c)),
            (gates.H, (b,)),
            (gates.CNOT, (a, b)),
            (gates.R, (c,)),
            (gates.CNOT, (a, b)),
            (gates.R, (a,)),
            (gates.CNOT, (a, b)),
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


EPS = 1e-6


def main():
    output_widget = some_widget()
    input_widget = some_widget()
    n_qubits = output_widget.get_n_qubits()
    max_qubits = output_widget.get_max_qubits()

    output_alias = WidgetAlias(output_widget)
    output_copy = WidgetPtrCopy(output_alias)

    input_clifford_buffer = CliffordBuffer(n_qubits)
    input_cliffords = LocalCliffords(
        input_clifford_buffer.n_qubits, input_clifford_buffer.cliffords
    )
    # print(input_cliffords.to_list())

    output_clifford_buffer = CliffordBuffer(n_qubits)
    output_cliffords = LocalCliffords(
        output_clifford_buffer.n_qubits, output_clifford_buffer.cliffords
    )

    # the local simulator is patched to include these corrections directly after the cz
    # operations
    input_widget.local_complement_corrections = input_cliffords
    output_widget.local_complement_corrections = output_cliffords

    artifacts = core_lib.core_lib.lcmh_search_with_defaults(
        n_qubits, output_copy.slices, output_cliffords.arr
    )


    print("initial cost", artifacts.initial_cost)
    print("output cost", artifacts.best_cost)

    input_widget.tableau_print()
    print()
    output_widget.tableau_print()
    print()
    print(output_cliffords.to_list(cache=False))

    for i in range(5):
    # for i in range(1):
        print(i)
        input_state = local_simulator.kr(
            local_simulator.state_prep(*list(np.random.random(2))),
            local_simulator.state_prep(*list(np.random.random(2))),
            local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.plus_state,
            # local_simulator.plus_state,
            # local_simulator.plus_state,
            # local_simulator.state_prep(*list(np.random.random(2))),
            # local_simulator.state_prep(*list(np.random.random(2))),
        )
        input_widget_state = local_simulator.simulate_widget(
            input_widget, input_state=input_state, trace_graph=True
        )
        output_widget_state = local_simulator.simulate_widget(
            output_widget, input_state=input_state, trace_graph=True
        )

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
