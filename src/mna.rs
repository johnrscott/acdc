use std::fmt;

use csuperlu::sparse_matrix::SparseMatrix;

use crate::{sparse::{concat_horizontal, concat_vertical,
		    transpose, neg, solve}, component::Component};

/// Matrix for modified nodal analysis
///
/// Stores the modified nodal analysis matrix
/// for a resistive network with no controlled,
/// sources, where group 2 contains no current
/// sources.
///
///  | A1 Y11 A1^T     A2  |
///  |                     |
///  |   - A2         Z22  |
/// 
///
pub struct MnaMatrix{ 
    top_left: SparseMatrix<f64>,
    top_right: SparseMatrix<f64>,
    bottom_right: SparseMatrix<f64>,
}

/// Modified nodal analysis right-hand side
///
/// The right-hand side for modified nodal analysis is
///
/// | -A1 s1 |
/// |        |
/// |   s2   |
///
pub struct MnaRhs {
    minus_a1_s1: Vec<f64>,
    s2: Vec<f64>,
}

pub struct Mna {
    matrix: MnaMatrix,
    rhs: MnaRhs,
}

impl Mna {
    pub fn new() -> Self {
	Self {
	    matrix: MnaMatrix::new(),
	    rhs: MnaRhs::new(),
	}
    }
    pub fn add_element_stamp(&mut self, component: Component) {
	match component {
	    Component::Resistor {
		term_1,
		term_2,
		resistance,
		group2,
	    } => {
		if group2 {
		    todo!("Not implemented yet")
		} else {
		    let r = resistance;
		    self.matrix.add_symmetric_group1(term_1, term_2, 1.0/r, -1.0/r);
		}
		println!("Element stamp for R")
	    },
	    Component::VoltageSource {
		term_pos,
		term_neg,
		voltage,
	    } => println!("Element stamp for V"),
	    _ => println!("Not currently implemented"),
	}
    }
}

impl fmt::Display for Mna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "MNA matrix:")?;
	writeln!(f, "{}", self.matrix)
    }
}

impl MnaMatrix {
    pub fn new() -> Self {
	Self {
	    // Insert some placeholder size here
	    top_left: SparseMatrix::new(),
	    top_right: SparseMatrix::new(),
	    bottom_right: SparseMatrix::new(),
	}
    }
    pub fn get_matrix(self) -> SparseMatrix<f64> {
	let top = concat_horizontal(self.top_left, &self.top_right);
	let bottom = concat_horizontal(neg(transpose(self.top_right)), &self.bottom_right);
	concat_vertical(top, &bottom)
    }

    /// Add a block of symmetric values to the top left matrix.
    ///
    /// The two indices specified defines a group of four matrix entries $(n_1-1, n_1-1) =
    /// (n_2-1,n_2-1) = x_1$, and $(n_1-1,n_2-1) = (n_2-1,n_1-1) = x_2$ (i.e. a symmetric block).
    /// Indices $n1$ and $n2$ are non-zero, and must be different. If either
    /// $n_1 = 0$ or $n_2 = 0$, then any elements where the matrix index would
    /// be negative are not written.
    ///
    /// This matrix block is added to the current matrix in the top left of the MNA matrix.
    pub fn add_symmetric_group1(&mut self, n1: usize, n2: usize, x1: f64, x2: f64) {
	if n1 == n2 {
	    panic!("Cannot set symmetric group where n1 == n2");
	}
	if n1 == 0 {
	    self.top_left.set_value(n2 - 1, n2 - 1, x1 + self.top_left.get_value(n2 - 1, n2 - 1));
	} else if n2 == 0 {
	    self.top_left.set_value(n1 - 1, n1 - 1, x1 + self.top_left.get_value(n1 - 1, n1 - 1));
 	} else {
	    self.top_left.set_value(n1 - 1, n1 - 1, x1 + self.top_left.get_value(n1 - 1, n1 - 1));
	    self.top_left.set_value(n2 - 1, n2 - 1, x1 + self.top_left.get_value(n2 - 1, n2 - 1));
	    self.top_left.set_value(n1 - 1, n2 - 1, x2 + self.top_left.get_value(n1 - 1, n2 - 1));
	    self.top_left.set_value(n2 - 1, n1 - 1, x2 + self.top_left.get_value(n2 - 1, n1 - 1));
	}
    }
}

impl fmt::Display for MnaMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "A1 Y11 A1^T:")?;
	writeln!(f, "{}", self.top_left)?;
	writeln!(f, "A2:")?;
	writeln!(f, "{}", self.top_right)?;
	writeln!(f, "Z22:")?;
	writeln!(f, "{}", self.bottom_right)
    }
}

impl MnaRhs {
    fn new() -> Self {
	Self {
	    minus_a1_s1: Vec::new(),
	    s2: Vec::new(),
	}
    }
    pub fn get_vector(mut self) -> Vec<f64> {
	self.minus_a1_s1.append(&mut self.s2);
	self.minus_a1_s1
    }
}

fn solve_mna(matrix: MnaMatrix, rhs: MnaRhs) -> Vec<f64> { 
    solve(matrix.get_matrix(), rhs.get_vector())
}
