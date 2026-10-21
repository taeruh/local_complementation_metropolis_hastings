use crate::{graph::Graph, one_local_clifford::Clifford};

// encoded as g = i^phase * Z * X
#[derive(Debug, PartialEq)]
pub struct Pauli {
    n: usize,
    g: Vec<bool>,
    phase: u8,
}

impl Pauli {
    fn new(z: Vec<bool>, x: Vec<bool>, phase: u8) -> Self {
        assert_eq!(z.len(), x.len(), "z and x must have the same length");
        let n = z.len();
        let mut g = z;
        g.extend(x);
        Self { n, g, phase }
    }

    fn mut_both(&mut self) -> (&mut [bool], &mut [bool]) {
        self.g.split_at_mut(self.n)
    }

    fn z(&self) -> &[bool] {
        &self.g[..self.n]
    }

    fn x(&self) -> &[bool] {
        &self.g[self.n..]
    }

    fn both(&self) -> (&[bool], &[bool]) {
        self.g.split_at(self.n)
    }

    // multiply other from the right
    fn multiply(&mut self, other: &Pauli) {
        let mut phase = 0;
        let n = self.n;
        let (self_z, self_x) = self.mut_both();
        let (other_z, other_x) = other.both();
        for i in 0..n {
            if self_x[i] && other_z[i] {
                phase = (phase + 2) % 4;
            }
            self_z[i] ^= other_z[i];
            self_x[i] ^= other_x[i];
        }
        self.phase = (self.phase + other.phase + phase) % 4;
    }

    fn commute(&self, other: &Pauli) -> bool {
        let n = self.n;
        let (self_z, self_x) = self.both();
        let (other_z, other_x) = other.both();
        let mut commutator = 0;
        for i in 0..n {
            if self_z[i] && other_x[i] {
                commutator += 1;
            }
            if self_x[i] && other_z[i] {
                commutator += 1;
            }
        }
        commutator % 2 == 0
    }

    fn con_z(&mut self, bit: usize) {
        if self.x()[bit] {
            self.phase = (self.phase + 2) % 4;
        }
    }

    fn con_x(&mut self, bit: usize) {
        if self.z()[bit] {
            self.phase = (self.phase + 2) % 4;
        }
    }

    fn con_y(&mut self, bit: usize) {
        self.con_z(bit);
        self.con_x(bit);
    }

    fn con_h(&mut self, bit: usize) {
        let (z, x) = self.mut_both();
        let z_bit = z[bit];
        let x_bit = x[bit];
        z[bit] = x_bit;
        x[bit] = z_bit;
        if z_bit && x_bit {
            self.phase = (self.phase + 2) % 4;
        }
    }

    fn con_s(&mut self, bit: usize) {
        let (z, x) = self.mut_both();
        if x[bit] {
            z[bit] ^= true;
            self.phase = (self.phase + 3) % 4;
        }
    }

    fn con_sh(&mut self, bit: usize) {
        self.con_h(bit);
        self.con_s(bit);
    }

    fn con_hs(&mut self, bit: usize) {
        self.con_s(bit);
        self.con_h(bit);
    }

    fn con_hsh(&mut self, bit: usize) {
        self.con_h(bit);
        self.con_s(bit);
        self.con_h(bit);
    }

    fn con(&mut self, clifford: Clifford, bit: usize) {
        match clifford {
            Clifford::I => {},
            Clifford::X => self.con_x(bit),
            Clifford::Y => self.con_y(bit),
            Clifford::Z => self.con_z(bit),
            Clifford::H => self.con_h(bit),
            Clifford::HX => {
                self.con_x(bit);
                self.con_h(bit);
            },
            Clifford::HY => {
                self.con_y(bit);
                self.con_h(bit);
            },
            Clifford::HZ => {
                self.con_z(bit);
                self.con_h(bit);
            },
            Clifford::S => self.con_s(bit),
            Clifford::SX => {
                self.con_x(bit);
                self.con_s(bit);
            },
            Clifford::SY => {
                self.con_y(bit);
                self.con_s(bit);
            },
            Clifford::SZ => {
                self.con_z(bit);
                self.con_s(bit);
            },
            Clifford::SH => self.con_sh(bit),
            Clifford::SHX => {
                self.con_x(bit);
                self.con_sh(bit);
            },
            Clifford::SHY => {
                self.con_y(bit);
                self.con_sh(bit);
            },
            Clifford::SHZ => {
                self.con_z(bit);
                self.con_sh(bit);
            },
            Clifford::HS => self.con_hs(bit),
            Clifford::HSX => {
                self.con_x(bit);
                self.con_hs(bit);
            },
            Clifford::HSY => {
                self.con_y(bit);
                self.con_hs(bit);
            },
            Clifford::HSZ => {
                self.con_z(bit);
                self.con_hs(bit);
            },
            Clifford::HSH => self.con_hsh(bit),
            Clifford::HSHX => {
                self.con_x(bit);
                self.con_hsh(bit);
            },
            Clifford::HSHY => {
                self.con_y(bit);
                self.con_hsh(bit);
            },
            Clifford::HSHZ => {
                self.con_z(bit);
                self.con_hsh(bit);
            },
        }
    }
}

fn all_commute(paulis: &[Pauli]) -> bool {
    let n = paulis.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if !paulis[i].commute(&paulis[j]) {
                return false;
            }
        }
    }
    true
}

fn gaussian_elimination(paulis: &mut [Pauli]) {
    if paulis.is_empty() {
        return;
    }
    let n = paulis.len();
    assert_eq!(paulis[0].n, n, "pauli components must be square");
    let mut current_row = 0;
    for col in 0..(2 * n) {
        let mut found_entry = false;
        for row in current_row..n {
            if paulis[row].g[col] {
                paulis.swap(row, current_row);
                found_entry = true;
                break;
            }
        }
        if !found_entry {
            continue;
        }
        let (pauli_current_row, pauli_below) = paulis.split_at_mut(current_row + 1);
        for pauli in pauli_below.iter_mut() {
            if pauli.g[col] {
                pauli.multiply(&pauli_current_row[current_row]);
            }
        }
        let (pauli_above, pauli_current_row) = paulis.split_at_mut(current_row);
        for pauli in pauli_above.iter_mut() {
            if pauli.g[col] {
                pauli.multiply(&pauli_current_row[0]);
            }
        }
        current_row += 1;
        if current_row == n {
            break;
        }
    }
    assert_eq!(current_row, n, "paulis must be full rank");
}

// assuming the graph is encoded on the z-block
fn graph_to_paulis(graph: &Graph) -> Vec<Pauli> {
    let n = graph.num_nodes();
    let mut paulis = Vec::with_capacity(n);
    for i in 0..n {
        let mut z = vec![false; n];
        let mut x = vec![false; n];
        x[i] = true;
        for &j in graph.get_neighbours(i).unwrap() {
            z[j] = true;
        }
        paulis.push(Pauli::new(z, x, 0));
    }
    paulis
}

fn apply_cliffords_to_paulis(paulis: &mut [Pauli], cliffords: &[Clifford]) {
    let n = paulis.len();
    assert_eq!(n, cliffords.len(), "paulis and cliffords must have the same length");
    for pauli in paulis.iter_mut() {
        for (bit, &clifford) in cliffords.iter().enumerate() {
            pauli.con(clifford, bit);
        }
    }
}

pub fn reduce(graph: &Graph, cliffords: &[Clifford]) -> Vec<Pauli> {
    let mut paulis = graph_to_paulis(graph);
    assert!(all_commute(&paulis), "paulis must all commute");
    apply_cliffords_to_paulis(&mut paulis, cliffords);
    assert!(all_commute(&paulis), "paulis must all commute");
    gaussian_elimination(&mut paulis);
    assert!(all_commute(&paulis), "paulis must all commute");
    paulis
}
