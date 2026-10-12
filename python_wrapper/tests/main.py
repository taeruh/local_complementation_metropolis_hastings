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
            (gates.H, (targ,)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.RZ, (ctrl_b, _Tdag_)),
            (gates.RZ, (targ, _T_)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.CNOT, (ctrl_a, ctrl_b)),
            (gates.RZ, (targ, _Tdag_)),
            (gates.CNOT, (ctrl_b, targ)),
            (gates.H, (targ,)),
        ]
    toffoli = toffoli_gate(0, 1, 2)
    wid = Widget(n_qubits, max_qubits)  # , teleport_input=False)
    ops = OperationSequence(len(toffoli))
    for opcode, args in toffoli:
        ops.append(opcode, *args)
    wid(ops)
    return wid

def buffer_widget(n_qubits, max_qubits) -> Widget:
    return Widget(n_qubits, max_qubits)


if __name__ == "__main__":
    input_widget = toffoli_widget()
    output_widget = buffer_widget(3, 20)

    n_qubits = input_widget.widget.contents.max_qubits
    slices = input_widget.widget
    print(n_qubits)
    print(slices.contents.__tableau.contents)

    input_widget.tableau_print()

    # TODO: ...

    # core_lib.core_lib.lcmh_simpler_direct_search(
    #     # arguments...
    # )
