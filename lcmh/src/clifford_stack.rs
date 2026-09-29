// remark on the local complementation operators:
// - they are defined as square roots sqrt{-iX} and sqrt{iZ}
// - not all papers tell you which root to take (e.g., S and S^dagger both square to Z,
//   but they act slightly differently on stabilisers)
// - the roots are defined as sqrt(-iX) = exp(-i pi/4 X) and sqrt(iZ) = exp(i pi/4 Z)
//   (see, for example, https://iopscience.iop.org/article/10.1088/1367-2630/ae02bd)
// - up to phases, this gives sqrt{-iX} = HSH and sqrt{iZ} = S^dagger = "R" = SZ

use crate::c_interface::{LcmhSingleQubitCliffordOperation, clifford_ops};

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone)]
struct Pauli(u8);

// those are all the non-Pauli Cliffords that are relevant
#[derive(PartialEq, Eq, Copy, Clone)]
#[cfg_attr(test, derive(Hash))]
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Clifford {
    I,
    S,
    H,
    SH,
    HS,
    HSH, // equivalent to SHS for Pauli conjugation purposes
}

#[cfg_attr(test, derive(Clone, PartialEq, Eq, Debug))]
pub struct CliffordStack {
    clifford: Clifford,
    pauli: Pauli,
    prev_state: (Clifford, Pauli),
}

impl Pauli {
    const I: Pauli = Pauli(0);
    const Z: Pauli = Pauli(1);
    const X: Pauli = Pauli(2);
    const Y: Pauli = Pauli(3);

    fn multiply(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    fn conj_s(&mut self) {
        self.0 ^= (self.0 & 2) >> 1;
    }

    fn conj_hsh(&mut self) {
        self.0 ^= (self.0 & 1) << 1;
    }
}

impl Clifford {
    fn multiply(&mut self, other: Self) -> Pauli {
        match (*self, other) {
            (Clifford::I, _) => {
                *self = other;
                Pauli::I
            },
            (Clifford::S, Clifford::S) => {
                *self = Clifford::I;
                Pauli::Z
            },
            (Clifford::S, Clifford::HSH) => {
                // TODO: double check this
                *self = Clifford::HS;
                Pauli::Z
            },
            (Clifford::H, Clifford::S) => {
                *self = Clifford::HS;
                Pauli::I
            },
            (Clifford::H, Clifford::HSH) => {
                *self = Clifford::SH;
                Pauli::I
            },
            (Clifford::SH, Clifford::S) => {
                // TODO: double check this
                *self = Clifford::HSH;
                Pauli::X
            },
            (Clifford::SH, Clifford::HSH) => {
                *self = Clifford::H;
                Pauli::X
            },
            (Clifford::HS, Clifford::S) => {
                *self = Clifford::H;
                Pauli::Z
            },
            (Clifford::HS, Clifford::HSH) => {
                // TODO: double check this
                *self = Clifford::S;
                Pauli::Z
            },
            (Clifford::HSH, Clifford::S) => {
                // TODO: double check this
                *self = Clifford::SH;
                Pauli::X
            },
            (Clifford::HSH, Clifford::HSH) => {
                *self = Clifford::I;
                Pauli::X
            },
            other => {
                unreachable!(
                    "We should not need the Clifford multiplication for {:?} and {:?}",
                    other.0,
                    other.1
                )
            },
        }
    }
}

impl CliffordStack {
    pub fn new() -> Self {
        CliffordStack {
            clifford: Clifford::I,
            pauli: Pauli::I,
            prev_state: (Clifford::I, Pauli::I),
        }
    }

    pub fn into_cabaliser_encoding(
        self,
        node: usize,
    ) -> (
        Option<LcmhSingleQubitCliffordOperation>,
        Option<LcmhSingleQubitCliffordOperation>,
    ) {
        let clifford = match self.clifford {
            Clifford::I => clifford_ops::_I_,
            Clifford::S => clifford_ops::_S_,
            Clifford::H => clifford_ops::_H_,
            Clifford::SH => clifford_ops::_SH_,
            Clifford::HS => clifford_ops::_HS_,
            Clifford::HSH => clifford_ops::_HSH_,
        };
        let pauli = match self.pauli {
            Pauli::I => clifford_ops::_I_,
            Pauli::Z => clifford_ops::_Z_,
            Pauli::X => clifford_ops::_X_,
            Pauli::Y => clifford_ops::_Y_,
            _ => unreachable!(),
        };
        (
            Some(LcmhSingleQubitCliffordOperation { operation: clifford, node }),
            Some(LcmhSingleQubitCliffordOperation { operation: pauli, node }),
        )
    }

    pub fn push_r(&mut self) {
        self.prev_state = (self.clifford, self.pauli);
        self.pauli.conj_s();
        self.pauli.multiply(Pauli::Z);
        let pauli = self.clifford.multiply(Clifford::S);
        self.pauli.multiply(pauli);
    }

    pub fn push_hsh(&mut self) {
        self.prev_state = (self.clifford, self.pauli);
        self.pauli.conj_hsh();
        let pauli = self.clifford.multiply(Clifford::HSH);
        self.pauli.multiply(pauli);
    }

    pub fn reset_push(&mut self) {
        self.clifford = self.prev_state.0;
        self.pauli = self.prev_state.1;
    }

    #[cfg(test)]
    pub fn top_is_identity(&self) -> bool {
        self.clifford == Clifford::I && self.pauli == Pauli::I
    }

    pub fn num_non_pauli_cliffords(&self) -> usize {
        match self.clifford {
            Clifford::I => 0,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        f64::consts::PI,
        ops::{AddAssign, Mul, Sub},
    };

    use super::*;

    #[derive(Copy, Clone, Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl Mul for Complex {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Complex {
                real: self.real * rhs.real - self.imag * rhs.imag,
                imag: self.real * rhs.imag + self.imag * rhs.real,
            }
        }
    }

    impl AddAssign for Complex {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl Sub for Complex {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Complex {
                real: self.real - rhs.real,
                imag: self.imag - rhs.imag,
            }
        }
    }

    impl Complex {
        fn abs(&self) -> f64 {
            (self.real.powi(2) + self.imag.powi(2)).sqrt()
        }
    }

    #[derive(Copy, Clone)]
    struct Matrix {
        data: [[Complex; 2]; 2],
    }

    impl Matrix {
        fn multiply(&self, other: &Matrix) -> Matrix {
            let mut result = [[Complex { real: 0.0, imag: 0.0 }; 2]; 2];
            for (i, row) in result.iter_mut().enumerate() {
                for (j, entry) in row.iter_mut().enumerate() {
                    for k in 0..2 {
                        *entry += self.data[i][k] * other.data[k][j];
                    }
                }
            }
            Matrix { data: result }
        }

        fn multiply_scalar(&self, scalar: Complex) -> Matrix {
            let mut result = [[Complex { real: 0.0, imag: 0.0 }; 2]; 2];
            for (i, row) in result.iter_mut().enumerate() {
                for (j, entry) in row.iter_mut().enumerate() {
                    *entry = self.data[i][j] * scalar;
                }
            }
            Matrix { data: result }
        }

        fn is_equal(&self, other: &Matrix) -> bool {
            for i in 0..2 {
                for j in 0..2 {
                    if (self.data[i][j] - other.data[i][j]).abs() > 1e-10 {
                        return false;
                    }
                }
            }
            true
        }

        fn is_equal_up_to_phase(&self, other: &Matrix) -> bool {
            for i in 0..16 {
                let angle = PI * (i as f64) / 8.0;
                let phase = Complex {
                    real: angle.cos(),
                    imag: angle.sin(),
                };
                let scaled = self.multiply_scalar(phase);
                if scaled.is_equal(other) {
                    return true;
                }
            }
            false
        }
    }

    const PAULI_I: Matrix = Matrix {
        data: [
            [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
            [Complex { real: 0.0, imag: 0.0 }, Complex { real: 1.0, imag: 0.0 }],
        ],
    };
    const PAULI_X: Matrix = Matrix {
        data: [
            [Complex { real: 0.0, imag: 0.0 }, Complex { real: 1.0, imag: 0.0 }],
            [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
        ],
    };
    const PAULI_Y: Matrix = Matrix {
        data: [
            [Complex { real: 0.0, imag: 0.0 }, Complex { real: 0.0, imag: -1.0 }],
            [Complex { real: 0.0, imag: 1.0 }, Complex { real: 0.0, imag: 0.0 }],
        ],
    };
    const PAULI_Z: Matrix = Matrix {
        data: [
            [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
            [Complex { real: 0.0, imag: 0.0 }, Complex { real: -1.0, imag: 0.0 }],
        ],
    };

    fn pauli_matrix_map(pauli: Pauli) -> Matrix {
        match pauli {
            Pauli::I => PAULI_I,
            Pauli::X => PAULI_X,
            Pauli::Y => PAULI_Y,
            Pauli::Z => PAULI_Z,
            _ => unreachable!(),
        }
    }

    struct CliffordMatrixMap {
        map: HashMap<Clifford, Matrix>,
    }

    impl CliffordMatrixMap {
        #[allow(non_snake_case)]
        fn new() -> Self {
            let I = Matrix {
                data: [
                    [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
                    [Complex { real: 0.0, imag: 0.0 }, Complex { real: 1.0, imag: 0.0 }],
                ],
            };
            let S = Matrix {
                data: [
                    [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
                    [Complex { real: 0.0, imag: 0.0 }, Complex { real: 0.0, imag: 1.0 }],
                ],
            };
            let H = Matrix {
                data: [
                    [
                        Complex {
                            real: 1.0 / 2f64.sqrt(),
                            imag: 0.0,
                        },
                        Complex {
                            real: 1.0 / 2f64.sqrt(),
                            imag: 0.0,
                        },
                    ],
                    [
                        Complex {
                            real: 1.0 / 2f64.sqrt(),
                            imag: 0.0,
                        },
                        Complex {
                            real: -1.0 / 2f64.sqrt(),
                            imag: 0.0,
                        },
                    ],
                ],
            };

            let mut map = HashMap::new();
            map.insert(Clifford::I, I);
            map.insert(Clifford::SH, S.multiply(&H));
            map.insert(Clifford::HS, H.multiply(&S));
            map.insert(Clifford::HSH, H.multiply(&S).multiply(&H));
            map.insert(Clifford::S, S);
            map.insert(Clifford::H, H);

            Self { map }
        }
    }

    #[test]
    fn test_pushing() {
        let clifford_matrix_map = CliffordMatrixMap::new();

        for clifford in [
            Clifford::I,
            Clifford::S,
            Clifford::H,
            Clifford::SH,
            Clifford::HS,
            Clifford::HSH,
        ]
        .into_iter()
        {
            for pauli in [Pauli::I, Pauli::Z, Pauli::X, Pauli::Y].into_iter() {
                let stack = CliffordStack {
                    clifford,
                    pauli,
                    prev_state: (Clifford::I, Pauli::I),
                };
                let matrix = clifford_matrix_map
                    .map
                    .get(&clifford)
                    .unwrap()
                    .multiply(&pauli_matrix_map(pauli));
                for (op, op_matrix, op_desc) in [
                    (
                        &CliffordStack::push_r as &dyn Fn(&mut CliffordStack),
                        clifford_matrix_map
                            .map
                            .get(&Clifford::S)
                            .unwrap()
                            .multiply(&pauli_matrix_map(Pauli::Z)),
                        "push_r",
                    ),
                    (
                        &CliffordStack::push_hsh as &dyn Fn(&mut CliffordStack),
                        *clifford_matrix_map.map.get(&Clifford::HSH).unwrap(),
                        "push_hsh",
                    ),
                ] {
                    let matrix_result = matrix.multiply(&op_matrix);
                    let mut stack_copy = stack.clone();
                    op(&mut stack_copy);
                    let stack_matrix = clifford_matrix_map
                        .map
                        .get(&stack_copy.clifford)
                        .unwrap()
                        .multiply(&pauli_matrix_map(stack_copy.pauli));
                    assert!(
                        matrix_result.is_equal_up_to_phase(&stack_matrix),
                        "Failed for clifford {:?} and pauli {:?} with operation \
                         {:?}\nExpected matrix (up to phase):\n{:?}\nGot matrix:\n{:?}",
                        clifford,
                        pauli,
                        op_desc,
                        matrix_result.data,
                        stack_matrix.data
                    );
                }
            }
        }
    }

    // #[test]
    // fn pushs_inverse() {
    //     for lc_clifford in [Clifford::S, Clifford::HSH].into_iter() {
    //         for pauli in [Pauli::I, Pauli::Z, Pauli::X, Pauli::Y].into_iter() {
    //             let mut stack = CliffordStack {
    //                 bulk: vec![lc_clifford.clone()],
    //                 top: pauli,
    //                 last_poped: false,
    //             };
    //             let stack_copy = stack.clone();
    //             stack.push_r();
    //             stack.inverse_push_r();
    //             assert_eq!(stack, stack_copy);
    //             stack.push_hsh();
    //             stack.inverse_push_hsh();
    //             assert_eq!(stack, stack_copy);
    //         }
    //     }
    // }
}
