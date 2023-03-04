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
    bottom_left: SparseMatrix<f64>,
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
		current_index,
		resistance: r,
	    } => {
		match current_index {
		    Some(edge) => self.matrix.add_symmetric_group2(
			term_1, term_2, edge, 1.0, -1.0, -r),
		    None => self.matrix.add_symmetric_group1(term_1, term_2, 1.0/r, -1.0/r)
		}
		println!("Element stamp for R")
	    },
	    _ => todo!("Not currently implemented"),
	}
    }
}

impl fmt::Display for Mna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "MNA matrix:")?;
	writeln!(f, "{}", self.matrix)
    }
}

/// Assumes the matrix is square
fn plus_equals(mat: &mut SparseMatrix<f64>, row: usize, col: usize, val: f64) {
    let old_val = if (row < mat.num_rows()) && (col < mat.num_cols()) {
	mat.get_value(row, col)
    } else {
	0.0
    };
    mat.set_value(row, col, old_val + val);
}

impl MnaMatrix {
    pub fn new() -> Self {
	Self {
	    // Insert some placeholder size here
	    top_left: SparseMatrix::new(),
	    top_right: SparseMatrix::new(),
	    bottom_left: SparseMatrix::new(),
	    bottom_right: SparseMatrix::new(),
	}
    }
    pub fn get_matrix(self) -> SparseMatrix<f64> {
	let top = concat_horizontal(self.top_left, &self.top_right);
	let bottom = concat_horizontal(self.bottom_left, &self.bottom_right);
	concat_vertical(top, &bottom)
    }

    /// Add a block of symmetric values to the top-left matrix.
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
	    panic!("Cannot set symmetric group 1 where n1 == n2");
	}
	if n1 == 0 {
	    plus_equals(&mut self.top_left, n2 - 1, n2 - 1, x1);
	} else if n2 == 0 {
	    plus_equals(&mut self.top_left, n1 - 1, n1 - 1, x1);
 	} else {
	    plus_equals(&mut self.top_left, n1 - 1, n1 - 1, x1);
	    plus_equals(&mut self.top_left, n2 - 1, n2 - 1, x1);
	    plus_equals(&mut self.top_left, n1 - 1, n2 - 1, x2);
	    plus_equals(&mut self.top_left, n2 - 1, n1 - 1, x2);
	}
    }

    /// Add a symmetric component into the off-diagonal blocks and bottom-left matrix
    ///
    /// The function accumulates: $x_1$ to $(n_1-1, e)$ (top-right) and $(e, n_1-1)$
    /// (bottom-left); $x_2$ to $(n_2-1, e)$ (top-right) and $(e, n_2-1)$
    /// (bottom-left); and $y$ to $(e, e)$ (bottom-right).
    ///
    /// In all cases, if all cases, $n_1 != n_2$, and if $n_1 = 0$ or $n_2 = 0$, then
    /// the corresponding matrix entries are not written.
    pub fn add_symmetric_group2(
	&mut self, n1: usize, n2: usize, e: usize,
	x1: f64, x2: f64, y: f64
    ) {
	if n1 == n2 {
	    panic!("Cannot set symmetric group 2 where n1 == n2");
	}
	plus_equals(&mut self.bottom_right, e, e, x2);	
	if n1 != 0 {
	    plus_equals(&mut self.top_right, n1 - 1, e, x1);
	    plus_equals(&mut self.bottom_left, e, n1 - 1, x1);
	}
	if n2 != 0 {
	    plus_equals(&mut self.top_right, n2 - 1, e, x2);
	    plus_equals(&mut self.bottom_left, e, n2 - 1, x2);
	}
    }
}

impl fmt::Display for MnaMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "Top left:")?;
	writeln!(f, "{}", self.top_left)?;
	writeln!(f, "Top right:")?;
	writeln!(f, "{}", self.top_right)?;
	writeln!(f, "Bottom left:")?;
	writeln!(f, "{}", self.bottom_left)?;
	writeln!(f, "Bottom right:")?;
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
