// remark on the local complementation operators:
// - they are defined as square roots sqrt{-iX} and sqrt{iZ}
// - not all papers tell you which root to take (e.g., S and S^dagger both square to Z,
//   but they act slightly differently on stabilisers)
// - the roots are defined as sqrt(-iX) = exp(-i pi/4 X) and sqrt(iZ) = exp(i pi/4 Z)
//   (see, for example, https://iopscience.iop.org/article/10.1088/1367-2630/ae02bd)
// - up to phases, this gives sqrt{-iX} = HSH and sqrt{iZ} = S^dagger = "R" = SZ

// those are all the non-Pauli Cliffords that are relevant
#[derive(PartialEq, Eq, Copy, Clone)]
#[cfg_attr(test, derive(Hash))]
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
pub enum Clifford {
    I,
    X,
    Y,
    Z,
    S,
    SX,
    SY,
    SZ,
    H,
    HX,
    HY,
    HZ,
    SH,
    SHX,
    SHY,
    SHZ,
    HS,
    HSX,
    HSY,
    HSZ,
    HSH,
    HSHX,
    HSHY,
    HSHZ,
}

#[cfg_attr(test, derive(Clone, PartialEq, Eq, Debug))]
pub struct CliffordStack {
    clifford: Clifford,
    prev_state: Clifford,
}

impl Clifford {
    fn multiply_sz(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::SZ,
            Clifford::X => *self = Clifford::SX,
            Clifford::Y => *self = Clifford::SY,
            Clifford::Z => *self = Clifford::S,
            Clifford::S => *self = Clifford::I,
            Clifford::SX => *self = Clifford::Y,
            Clifford::SY => *self = Clifford::X,
            Clifford::SZ => *self = Clifford::Z,
            Clifford::H => *self = Clifford::HSZ,
            Clifford::HX => *self = Clifford::HSX,
            Clifford::HY => *self = Clifford::HSY,
            Clifford::HZ => *self = Clifford::HS,
            Clifford::SH => *self = Clifford::HSHY,
            Clifford::SHX => *self = Clifford::HSH,
            Clifford::SHY => *self = Clifford::HSHZ,
            Clifford::SHZ => *self = Clifford::HSHX,
            Clifford::HS => *self = Clifford::H,
            Clifford::HSX => *self = Clifford::HY,
            Clifford::HSY => *self = Clifford::HX,
            Clifford::HSZ => *self = Clifford::HZ,
            Clifford::HSH => *self = Clifford::SHY,
            Clifford::HSHX => *self = Clifford::SH,
            Clifford::HSHY => *self = Clifford::SHZ,
            Clifford::HSHZ => *self = Clifford::SHX,
        }
    }

    fn multiply_hsh(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::HSH,
            Clifford::X => *self = Clifford::HSHX,
            Clifford::Y => *self = Clifford::HSHZ,
            Clifford::Z => *self = Clifford::HSHY,
            Clifford::S => *self = Clifford::HSZ,
            Clifford::SX => *self = Clifford::HSY,
            Clifford::SY => *self = Clifford::HS,
            Clifford::SZ => *self = Clifford::HSX,
            Clifford::H => *self = Clifford::SH,
            Clifford::HX => *self = Clifford::SHX,
            Clifford::HY => *self = Clifford::SHZ,
            Clifford::HZ => *self = Clifford::SHY,
            Clifford::SH => *self = Clifford::HX,
            Clifford::SHX => *self = Clifford::H,
            Clifford::SHY => *self = Clifford::HY,
            Clifford::SHZ => *self = Clifford::HZ,
            Clifford::HS => *self = Clifford::SZ,
            Clifford::HSX => *self = Clifford::SY,
            Clifford::HSY => *self = Clifford::S,
            Clifford::HSZ => *self = Clifford::SX,
            Clifford::HSH => *self = Clifford::X,
            Clifford::HSHX => *self = Clifford::I,
            Clifford::HSHY => *self = Clifford::Y,
            Clifford::HSHZ => *self = Clifford::Z,
        }
    }
}

impl CliffordStack {
    pub fn new() -> Self {
        CliffordStack {
            clifford: Clifford::I,
            prev_state: Clifford::I,
        }
    }

    #[cfg(test)]
    fn new_with_clifford(clifford: Clifford) -> Self {
        CliffordStack { clifford, prev_state: clifford }
    }

    pub fn push_r(&mut self) {
        self.prev_state = self.clifford;
        self.clifford.multiply_sz();
    }

    pub fn push_hsh(&mut self) {
        self.prev_state = self.clifford;
        self.clifford.multiply_hsh();
    }

    pub fn reset_push(&mut self) {
        self.clifford = self.prev_state;
    }

    pub fn into_clifford(self) -> Clifford {
        self.clifford
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        f64::consts::PI,
        mem,
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
            let X = Matrix {
                data: [
                    [Complex { real: 0.0, imag: 0.0 }, Complex { real: 1.0, imag: 0.0 }],
                    [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
                ],
            };
            let Y = Matrix {
                data: [
                    [Complex { real: 0.0, imag: 0.0 }, Complex { real: 0.0, imag: -1.0 }],
                    [Complex { real: 0.0, imag: 1.0 }, Complex { real: 0.0, imag: 0.0 }],
                ],
            };
            let Z = Matrix {
                data: [
                    [Complex { real: 1.0, imag: 0.0 }, Complex { real: 0.0, imag: 0.0 }],
                    [Complex { real: 0.0, imag: 0.0 }, Complex { real: -1.0, imag: 0.0 }],
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

            let SH = S.multiply(&H);
            let HS = H.multiply(&S);
            let HSH = HS.multiply(&H);

            let mut map = HashMap::new();

            map.insert(Clifford::I, I);
            map.insert(Clifford::SX, S.multiply(&X));
            map.insert(Clifford::SY, S.multiply(&Y));
            map.insert(Clifford::SZ, S.multiply(&Z));
            map.insert(Clifford::S, S);
            map.insert(Clifford::HX, H.multiply(&X));
            map.insert(Clifford::HY, H.multiply(&Y));
            map.insert(Clifford::HZ, H.multiply(&Z));
            map.insert(Clifford::H, H);
            map.insert(Clifford::SHX, SH.multiply(&X));
            map.insert(Clifford::SHY, SH.multiply(&Y));
            map.insert(Clifford::SHZ, SH.multiply(&Z));
            map.insert(Clifford::SH, SH);
            map.insert(Clifford::HSX, HS.multiply(&X));
            map.insert(Clifford::HSY, HS.multiply(&Y));
            map.insert(Clifford::HSZ, HS.multiply(&Z));
            map.insert(Clifford::HS, HS);
            map.insert(Clifford::HSHX, HSH.multiply(&X));
            map.insert(Clifford::HSHY, HSH.multiply(&Y));
            map.insert(Clifford::HSHZ, HSH.multiply(&Z));
            map.insert(Clifford::HSH, HSH);
            map.insert(Clifford::X, X);
            map.insert(Clifford::Y, Y);
            map.insert(Clifford::Z, Z);

            Self { map }
        }
    }

    #[test]
    fn test_pushing() {
        let clifford_matrix_map = CliffordMatrixMap::new();

        for clifford_discriminant in 0u8..24 {
            // I think the following is safe but it is sketchy and I would not do it
            // ouside of a unit test.
            let clifford =
                // Safety: I fixed the Clifford enum to be repr(u8), it is a unit-only
                // enum, we don't specify the discriminant values, so they count up from
                // 0, and end at 23 as there are 24 variants.
                unsafe { mem::transmute::<u8, Clifford>(clifford_discriminant) };
            let stack = CliffordStack::new_with_clifford(clifford);
            let matrix = clifford_matrix_map.map.get(&clifford).unwrap();
            for (op, op_matrix, op_desc) in [
                (
                    &CliffordStack::push_r as &dyn Fn(&mut CliffordStack),
                    clifford_matrix_map
                        .map
                        .get(&Clifford::S)
                        .unwrap()
                        .multiply(clifford_matrix_map.map.get(&Clifford::Z).unwrap()),
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
                // println!("{:?}", stack_copy);
                let stack_matrix =
                    clifford_matrix_map.map.get(&stack_copy.clifford).unwrap();
                assert!(
                    matrix_result.is_equal_up_to_phase(stack_matrix),
                    "Failed for clifford {:?} with operation {:?}\nExpected matrix (up \
                     to phase):\n{:?}\nGot matrix:\n{:?}",
                    clifford,
                    op_desc,
                    matrix_result.data,
                    stack_matrix.data
                );
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
