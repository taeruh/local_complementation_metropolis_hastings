#!/usr/bin/env python

import ctypes
import numpy as np
from math import isnan
from itertools import chain
from lcmh import core_lib
from lcmh.core_lib import LcmhCoolingConfiguration
from lcmh import cabaliser_interface_helper
from lcmh.cabaliser_interface_helper import CliffordBuffer
from cabaliser.widget import Widget
from cabaliser.structs import WidgetType
from cabaliser import gates
from cabaliser.operation_sequence import OperationSequence
from cabaliser.io_array_wrappers import LocalCliffords


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

    _I_ = 0
    _T_ = 1
    _Tdag_ = 2
    def toffoli_gate(ctrl_a, ctrl_b, targ):
        return [
            (gates.H, (targ,)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_a, targ)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_a, targ)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (ctrl_a, _T_)),
            (gates.RZ, (ctrl_b, _Tdag_)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.H, (targ,)),
        ]

    gate = some_gate(0, 1, 2)
    # gate = toffoli_gate(0, 1, 2)
    wid = Widget(3, 20)
    ops = OperationSequence(len(gate))
    for opcode, args in gate:
        ops.append(opcode, *args)
    wid(ops)
    wid.decompose(do_lcmh=False)
    return wid


def main():
    output_widget = some_widget()
    n_qubits = output_widget.get_n_qubits()

    input_widget = some_widget()
    slices = cabaliser_interface_helper.get_slices(output_widget)
    lc_corrections = output_widget.lc_corrections.arr

    cooling_config = LcmhCoolingConfiguration(
        betas=[1, 2, 3], num_steps_per_beta=[10, 20, 30]
    )

    artifacts = core_lib.lcmh_search_with_defaults_except_cooling_config(
        n_qubits, slices, lc_corrections, cooling_config
    )

    print("Input Widget Tableau:")
    input_widget.tableau_print()
    print("Output Widget Tableau:")
    output_widget.tableau_print()
    print("Local complementation corrections after graph initialisation:")
    print(output_widget.lc_corrections.to_list())
    print("Initial cost", artifacts.initial_cost)
    print("Output cost", artifacts.best_cost)


if __name__ == "__main__":
    main()
