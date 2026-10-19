#!/usr/bin/env python

import ctypes
import numpy as np
from math import isnan
from itertools import chain
from lcmh import core_lib
from lcmh.core_lib import LcmhCoolingConfiguration
from lcmh.cabaliser_interface_helper import WidgetAlias, WidgetPtrCopy
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
    wid = Widget(3, 10)
    ops = OperationSequence(len(gate))
    for opcode, args in gate:
        ops.append(opcode, *args)
    wid(ops)
    wid.decompose()
    return wid


def buffer_widget(n_qubits, max_qubits) -> Widget:
    wid = Widget(n_qubits, max_qubits, n_inputs=0)
    wid.decompose()
    return wid


def main():
    input_widget = some_widget()
    n_qubits = input_widget.get_n_qubits()
    max_qubits = input_widget.get_max_qubits()
    output_widget = buffer_widget(n_qubits, max_qubits)

    input_alias = WidgetAlias(input_widget)
    output_alias = WidgetAlias(output_widget)
    input_copy = WidgetPtrCopy(input_alias)
    output_copy = WidgetPtrCopy(output_alias)

    cooling_config = LcmhCoolingConfiguration(
        betas=[1, 2, 3], num_steps_per_beta=[10, 20, 30]
    )

    artifacts = core_lib.lcmh_search_with_defaults_except_cooling_config(
        n_qubits,
        input_copy.slices,
        input_copy.cliffords,
        output_copy.slices,
        output_copy.cliffords,
        cooling_config
    )


    print("Input Widget Tableau:")
    input_widget.tableau_print()
    print("Output Widget Tableau:")
    output_widget.tableau_print()
    output_widget.local_cliffords = None
    print("Input Widget Local Cliffords:")
    print(input_widget.get_local_cliffords().to_list())
    print("Output Widget Local Cliffords:")
    print(output_widget.get_local_cliffords().to_list())
    print("initial cost", artifacts.initial_cost)
    print("output cost", artifacts.best_cost)

if __name__ == "__main__":
    main()
