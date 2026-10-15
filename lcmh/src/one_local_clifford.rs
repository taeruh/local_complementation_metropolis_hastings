// remark on the local complementation operators:
// - they are defined as square roots sqrt{-iX} and sqrt{iZ}
// - not all papers tell you which root to take (e.g., S and S^dagger both square to Z,
//   but they act slightly differently on stabilisers)
// - the roots are defined as sqrt(-iX) = exp(-i pi/4 X) and sqrt(iZ) = exp(i pi/4 Z)
//   (see, for example, https://iopscience.iop.org/article/10.1088/1367-2630/ae02bd)
// - up to phases, this gives sqrt{-iX} = HSH and sqrt{iZ} = S^dagger = "R" = SZ
// - because we want to correct these actions we have to multiply by the inverse (HSHX and
//   S) from the left in our clifford stack

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

impl Clifford {
    pub fn multiply_s_from_left(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::S,
            Clifford::X => *self = Clifford::SX,
            Clifford::Y => *self = Clifford::SY,
            Clifford::Z => *self = Clifford::SZ,
            Clifford::S => *self = Clifford::Z,
            Clifford::SX => *self = Clifford::Y,
            Clifford::SY => *self = Clifford::X,
            Clifford::SZ => *self = Clifford::I,
            Clifford::H => *self = Clifford::SH,
            Clifford::HX => *self = Clifford::SHX,
            Clifford::HY => *self = Clifford::SHY,
            Clifford::HZ => *self = Clifford::SHZ,
            Clifford::SH => *self = Clifford::HX,
            Clifford::SHX => *self = Clifford::H,
            Clifford::SHY => *self = Clifford::HZ,
            Clifford::SHZ => *self = Clifford::HY,
            Clifford::HS => *self = Clifford::HSHX,
            Clifford::HSX => *self = Clifford::HSH,
            Clifford::HSY => *self = Clifford::HSHZ,
            Clifford::HSZ => *self = Clifford::HSHY,
            Clifford::HSH => *self = Clifford::HSZ,
            Clifford::HSHX => *self = Clifford::HSY,
            Clifford::HSHY => *self = Clifford::HSX,
            Clifford::HSHZ => *self = Clifford::HS,
        }
    }

    pub fn multiply_hshx_from_left(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::HSHX,
            Clifford::X => *self = Clifford::HSH,
            Clifford::Y => *self = Clifford::HSHZ,
            Clifford::Z => *self = Clifford::HSHY,
            Clifford::S => *self = Clifford::SHZ,
            Clifford::SX => *self = Clifford::SHY,
            Clifford::SY => *self = Clifford::SHX,
            Clifford::SZ => *self = Clifford::SH,
            Clifford::H => *self = Clifford::HSZ,
            Clifford::HX => *self = Clifford::HSY,
            Clifford::HY => *self = Clifford::HSX,
            Clifford::HZ => *self = Clifford::HS,
            Clifford::SH => *self = Clifford::SX,
            Clifford::SHX => *self = Clifford::S,
            Clifford::SHY => *self = Clifford::SZ,
            Clifford::SHZ => *self = Clifford::SY,
            Clifford::HS => *self = Clifford::H,
            Clifford::HSX => *self = Clifford::HX,
            Clifford::HSY => *self = Clifford::HY,
            Clifford::HSZ => *self = Clifford::HZ,
            Clifford::HSH => *self = Clifford::I,
            Clifford::HSHX => *self = Clifford::X,
            Clifford::HSHY => *self = Clifford::Y,
            Clifford::HSHZ => *self = Clifford::Z,
        }
    }

    pub fn multiply_h_from_right(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::H,
            Clifford::X => *self = Clifford::HZ,
            Clifford::Y => *self = Clifford::HY,
            Clifford::Z => *self = Clifford::HX,
            Clifford::S => *self = Clifford::SH,
            Clifford::SX => *self = Clifford::SHZ,
            Clifford::SY => *self = Clifford::SHY,
            Clifford::SZ => *self = Clifford::SHX,
            Clifford::H => *self = Clifford::I,
            Clifford::HX => *self = Clifford::Z,
            Clifford::HY => *self = Clifford::Y,
            Clifford::HZ => *self = Clifford::X,
            Clifford::SH => *self = Clifford::S,
            Clifford::SHX => *self = Clifford::SZ,
            Clifford::SHY => *self = Clifford::SY,
            Clifford::SHZ => *self = Clifford::SX,
            Clifford::HS => *self = Clifford::HSH,
            Clifford::HSX => *self = Clifford::HSHZ,
            Clifford::HSY => *self = Clifford::HSHY,
            Clifford::HSZ => *self = Clifford::HSHX,
            Clifford::HSH => *self = Clifford::HS,
            Clifford::HSHX => *self = Clifford::HSZ,
            Clifford::HSHY => *self = Clifford::HSY,
            Clifford::HSHZ => *self = Clifford::HSX,
        }
    }

    pub fn multiply_z_from_right(&mut self) {
        match *self {
            Clifford::I => *self = Clifford::Z,
            Clifford::X => *self = Clifford::Y,
            Clifford::Y => *self = Clifford::X,
            Clifford::Z => *self = Clifford::I,
            Clifford::S => *self = Clifford::SZ,
            Clifford::SX => *self = Clifford::SY,
            Clifford::SY => *self = Clifford::SX,
            Clifford::SZ => *self = Clifford::S,
            Clifford::H => *self = Clifford::HZ,
            Clifford::HX => *self = Clifford::HY,
            Clifford::HY => *self = Clifford::HX,
            Clifford::HZ => *self = Clifford::H,
            Clifford::SH => *self = Clifford::SHZ,
            Clifford::SHX => *self = Clifford::SHY,
            Clifford::SHY => *self = Clifford::SHX,
            Clifford::SHZ => *self = Clifford::SH,
            Clifford::HS => *self = Clifford::HSZ,
            Clifford::HSX => *self = Clifford::HSY,
            Clifford::HSY => *self = Clifford::HSX,
            Clifford::HSZ => *self = Clifford::HS,
            Clifford::HSH => *self = Clifford::HSHZ,
            Clifford::HSHX => *self = Clifford::HSHY,
            Clifford::HSHY => *self = Clifford::HSHX,
            Clifford::HSHZ => *self = Clifford::HSH,
        }
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
    fn test_multiplications() {
        let clifford_matrix_map = CliffordMatrixMap::new();

        for clifford_discriminant in 0u8..24 {
            // I think the following is safe but it is sketchy and I would not do it
            // ouside of a unit test.
            let clifford =
                // Safety: I fixed the Clifford enum to be repr(u8), it is a unit-only
                // enum, we don't specify the discriminant values, so they count up from
                // 0, and end at 23 as there are 24 variants.
                unsafe { mem::transmute::<u8, Clifford>(clifford_discriminant) };
            let matrix = clifford_matrix_map.map.get(&clifford).unwrap();
            for (op, op_matrix, from_left, op_desc) in [
                (
                    &Clifford::multiply_s_from_left as &dyn Fn(&mut Clifford),
                    clifford_matrix_map.map.get(&Clifford::S).unwrap(),
                    true,
                    "s_from_left",
                ),
                (
                    &Clifford::multiply_hshx_from_left as &dyn Fn(&mut Clifford),
                    clifford_matrix_map.map.get(&Clifford::HSHX).unwrap(),
                    true,
                    "hshx_from_left",
                ),
                (
                    &Clifford::multiply_z_from_right as &dyn Fn(&mut Clifford),
                    clifford_matrix_map.map.get(&Clifford::Z).unwrap(),
                    false,
                    "z_from_right",
                ),
                (
                    &Clifford::multiply_h_from_right as &dyn Fn(&mut Clifford),
                    clifford_matrix_map.map.get(&Clifford::H).unwrap(),
                    false,
                    "h_from_right",
                ),
            ] {
                let matrix_result = if from_left {
                    op_matrix.multiply(matrix)
                } else {
                    matrix.multiply(op_matrix)
                };
                let mut clifford_copy = clifford;
                op(&mut clifford_copy);
                let clifford_matrix =
                    clifford_matrix_map.map.get(&clifford_copy).unwrap();
                assert!(
                    matrix_result.is_equal_up_to_phase(clifford_matrix),
                    "Failed for clifford {:?} with operation {:?}\nExpected matrix (up \
                     to phase):\n{:?}\nGot matrix:\n{:?}",
                    clifford,
                    op_desc,
                    matrix_result.data,
                    clifford_matrix.data
                );
            }
        }
    }
}
