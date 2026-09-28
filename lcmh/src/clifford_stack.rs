// remark on the local complementation operators:
// - they are defined as square roots sqrt{-iX} and sqrt{iZ}
// - not all papers tell you which root to take (e.g., S and S^dagger both square to Z,
//   but they act slightly differently on stabilisers)
// - the roots are defined as sqrt(-iX) = exp(-i pi/4 X) and sqrt(iZ) = exp(i pi/4 Z)
//   (see, for example, https://iopscience.iop.org/article/10.1088/1367-2630/ae02bd)
// - up to phases, this gives sqrt{-iX} = HSH and sqrt{iZ} = S^dagger = "R" = SZ

use crate::c_interface::{LcmhSingleQubitCliffordOperation, clifford_ops};

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Clone, Debug))]
struct Pauli(u8);

// those are all the non-Pauli Cliffords that are relevant
#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Clone, Debug))]
enum LcClifford {
    S,
    #[allow(clippy::upper_case_acronyms)]
    HSH, // equivalent to SHS for Pauli conjugation purposes
}

#[cfg_attr(test, derive(Clone, PartialEq, Eq, Debug))]
pub struct CliffordStack {
    bulk: Vec<LcClifford>,
    top: Pauli, // we can always keep the Pauli(s) on the top via conjugation
    // needed to invert the push operations (and with that invert the local
    // complementation)
    last_poped: bool,
}

impl Pauli {
    const I: Pauli = Pauli(0);
    const Z: Pauli = Pauli(1);
    const X: Pauli = Pauli(2);
    const Y: Pauli = Pauli(3);

    const fn multiply(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    const fn conj_s(&mut self) {
        self.0 ^= (self.0 & 2) >> 1;
    }

    const fn conj_hsh(&mut self) {
        self.0 ^= (self.0 & 1) << 1;
    }
}

impl CliffordStack {
    pub fn new() -> Self {
        CliffordStack {
            bulk: Vec::new(),
            top: Pauli::I,
            last_poped: false,
        }
    }

    pub fn into_cabaliser_encoding(
        self,
        node: usize,
    ) -> (
        impl Iterator<Item = LcmhSingleQubitCliffordOperation>,
        Option<LcmhSingleQubitCliffordOperation>,
    ) {
        let lc_cliffords =
            self.bulk.into_iter().map(move |op| LcmhSingleQubitCliffordOperation {
                operation: match op {
                    LcClifford::S => clifford_ops::_S_,
                    LcClifford::HSH => clifford_ops::_HSH_,
                },
                node,
            });
        let pauli = match self.top {
            Pauli::I => return (lc_cliffords, None),
            Pauli::Z => clifford_ops::_Z_,
            Pauli::X => clifford_ops::_X_,
            Pauli::Y => clifford_ops::_Y_,
            _ => unreachable!(),
        };
        (
            lc_cliffords,
            (Some(LcmhSingleQubitCliffordOperation { operation: pauli, node })),
        )
    }

    pub fn push_r(&mut self) {
        self.top.conj_s();
        if let Some(last) = self.bulk.last() {
            if *last == LcClifford::S {
                self.bulk.pop();
                self.last_poped = true;
                return;
            }
        }
        self.last_poped = false;
        self.bulk.push(LcClifford::S);
        self.top.multiply(Pauli::Z);
    }

    pub fn inverse_push_r(&mut self) {
        if self.last_poped {
            self.bulk.push(LcClifford::S);
            self.last_poped = false;
        } else {
            self.bulk.pop();
            self.top.multiply(Pauli::Z);
        }
        self.top.conj_s()
    }

    pub fn push_hsh(&mut self) {
        self.top.conj_hsh();
        if let Some(last) = self.bulk.last() {
            if *last == LcClifford::HSH {
                self.bulk.pop();
                self.top.multiply(Pauli::X);
                self.last_poped = true;
                return;
            }
        }
        self.bulk.push(LcClifford::HSH);
        self.last_poped = false;
    }

    pub fn inverse_push_hsh(&mut self) {
        if self.last_poped {
            self.bulk.push(LcClifford::HSH);
            self.top.multiply(Pauli::X);
            self.last_poped = false;
        } else {
            self.bulk.pop();
        }
        self.top.conj_hsh()
    }

    #[cfg(test)]
    pub fn top_is_identity(&self) -> bool {
        self.top == Pauli::I
    }

    pub fn num_non_pauli_cliffords(&self) -> usize {
        self.bulk.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushs_inverse() {
        for lc_clifford in [LcClifford::S, LcClifford::HSH].into_iter() {
            for pauli in [Pauli::I, Pauli::Z, Pauli::X, Pauli::Y].into_iter() {
                let mut stack = CliffordStack {
                    bulk: vec![lc_clifford.clone()],
                    top: pauli,
                    last_poped: false,
                };
                let stack_copy = stack.clone();
                stack.push_r();
                stack.inverse_push_r();
                assert_eq!(stack, stack_copy);
                stack.push_hsh();
                stack.inverse_push_hsh();
                assert_eq!(stack, stack_copy);
            }
        }
    }
}
