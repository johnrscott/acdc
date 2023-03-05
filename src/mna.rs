use std::fmt;
use std::cmp;

use csuperlu::sparse_matrix::SparseMatrix;

use crate::{sparse::{concat_horizontal, concat_vertical}, component::Component};

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
    /// The number of rows in the top matrices
    num_voltage_nodes: usize,
    /// The number of rows in the bottom matrices
    num_current_edges: usize,
    top_left: SparseMatrix<f64>,
    top_right: SparseMatrix<f64>,
    bottom_left: SparseMatrix<f64>,
    bottom_right: SparseMatrix<f64>,
}

impl MnaMatrix {
    pub fn new() -> Self {
	Self {
	    num_voltage_nodes: 0,
	    num_current_edges: 0,
	    top_left: SparseMatrix::new(),
	    top_right: SparseMatrix::new(),
	    bottom_left: SparseMatrix::new(),
	    bottom_right: SparseMatrix::new(),
	}
    }

    pub fn num_voltage_nodes(&self) -> usize {
	self.num_voltage_nodes
    }

    pub fn num_current_edges(&self) -> usize {
	self.num_current_edges
    }
    
    pub fn get_matrix(mut self) -> SparseMatrix<f64> {

	self.top_left.resize(self.num_voltage_nodes, self.num_voltage_nodes);
	self.bottom_right.resize(self.num_current_edges, self.num_current_edges);
	self.top_right.resize(self.num_voltage_nodes, self.num_current_edges);	
	self.bottom_left.resize(self.num_current_edges, self.num_voltage_nodes);

	let top = concat_horizontal(self.top_left, &self.top_right);
	let bottom = concat_horizontal(self.bottom_left, &self.bottom_right);
	concat_vertical(top, &bottom)
    }

    /// Increase the number of voltage nodes if n is not already included. Note
    /// that this function uses the netlist value of n (i.e. the matrix index is
    /// n-1). 
    fn update_num_voltage_nodes(&mut self, n: usize) {
	self.num_voltage_nodes = cmp::max(self.num_voltage_nodes, n);
    }

    /// Increase the number of current edges if e is not already included. Note that
    /// e is the actual index into the matrix, so the number of rows will be resized
    /// to e+1
    fn update_num_current_edges(&mut self, e: usize) {
	self.num_current_edges = cmp::max(self.num_current_edges, e + 1);
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
	self.update_num_voltage_nodes(n1);
	self.update_num_voltage_nodes(n2);
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
	self.update_num_voltage_nodes(n1);
	self.update_num_voltage_nodes(n2);
	self.update_num_current_edges(e);
	plus_equals(&mut self.bottom_right, e, e, y);	
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

/// Modified nodal analysis right-hand side
///
/// The right-hand side for modified nodal analysis is
///
/// | -A1 s1 |
/// |        |
/// |   s2   |
///
pub struct MnaRhs {
    top: SparseMatrix<f64>,
    bottom: SparseMatrix<f64>,
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

impl fmt::Display for MnaMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "Num voltage nodes = {}, Num current edges = {}",
		 self.num_voltage_nodes, self.num_current_edges)?;
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
	    top: SparseMatrix::new(),
	    bottom: SparseMatrix::new(),
	}
    }
    
    pub fn get_vector(self, num_voltage_nodes: usize, num_current_edges: usize) -> Vec<f64> {
	let mut out = vec![0.0; num_voltage_nodes + num_current_edges];
	for ((row, _), value) in self.top.values().iter() {
	    out[*row] = *value;
	}
	for ((row, _), value) in self.bottom.values().iter() {
	    out[num_voltage_nodes + *row] = *value;
	}
	out
    }

    /// Add a RHS element in the group 2 matrix
    pub fn add_rhs_stamp(&mut self, e: usize, x: f64) {
	self.bottom.set_value(e, 1, x);
    }
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

    pub fn num_voltage_nodes(&self) -> usize {
	self.matrix.num_voltage_nodes()
    }

    pub fn num_current_edges(&self) -> usize {
	self.matrix.num_current_edges()
    }
    
    pub fn add_element_stamp(&mut self, component: &Component) {
	match component {
	    Component::Resistor {
		term_1,
		term_2,
		current_index,
		resistance: r,
	    } => {
		println!("Adding volt stamp: {term_1}, {term_1}, {:?}", current_index);
		match current_index {
		    Some(edge) => self.matrix.add_symmetric_group2(
			*term_1, *term_2, *edge, 1.0, -1.0, -*r),
		    None => self.matrix.add_symmetric_group1(*term_1, *term_2, 1.0/r, -1.0/r)
		}
	    },
	    Component::IndependentVoltageSource {
		term_pos,
		term_neg,
		current_index,
		voltage: v,
	    } => {
		println!("Adding volt stamp: {term_pos}, {term_neg}, {current_index}");
		self.matrix.add_symmetric_group2(*term_pos, *term_neg, *current_index, 1.0, -1.0, 0.0);
		self.rhs.add_rhs_stamp(*current_index, *v);
	    },
	    _ => todo!("Not currently implemented"),
	}
    }

    /// Return (matrix, rhs)
    pub fn get_system(self) -> (SparseMatrix<f64>, Vec<f64>) {
	let num_voltage_nodes = self.matrix.num_voltage_nodes();
	let num_current_edges = self.matrix.num_current_edges();
	let matrix = self.matrix.get_matrix();
	let rhs = self.rhs.get_vector(num_voltage_nodes, num_current_edges);
	(matrix, rhs)
    }
}

impl fmt::Display for Mna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "MNA matrix:")?;
	writeln!(f, "{}", self.matrix)
    }
}


