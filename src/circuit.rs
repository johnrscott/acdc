use csuperlu::c::value_type::ValueType;
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
struct MnaMatrix<P: ValueType<P>> {
    a1_y11_a1t: SparseMatrix<P>,
    a2: SparseMatrix<P>,
    z22: SparseMatrix<P>,
}

/// Modified nodal analysis right-hand side
///
/// The right-hand side for modified nodal analysis is
///
/// | -A1 s1 |
/// |        |
/// |   s2   |
///
struct MnaRhs<P: ValueType<P>> {
    minus_a1_s1: Vec<P>,
    s2: Vec<P>,
}

impl<P: ValueType<P>> MnaMatrix<P> {
    pub fn new() -> Self {
	Self {
	    // Insert some placeholder size here
	    a1_y11_a1t: SparseMatrix::new(0, 0),
	    a2: SparseMatrix::new(0, 0),
	    z22: SparseMatrix::new(0, 0),
	}
    }
    pub fn get_matrix(self) -> SparseMatrix<P> {
	let top = concat_horizontal(self.a1_y11_a1t, &self.a2);
	let bottom = concat_horizontal(neg(transpose(self.a2)), &self.z22);
	concat_vertical(top, &bottom)
    }
}

impl<P: ValueType<P>> MnaRhs<P> {
    fn new() -> Self {
	Self {
	    minus_a1_s1: Vec::new(),
	    s2: Vec::new(),
	}
    }
    pub fn get_vector(mut self) -> Vec<P> {
	self.minus_a1_s1.append(&mut self.s2);
	self.minus_a1_s1
    }
}

fn solve_mna<P: ValueType<P>>(matrix: MnaMatrix<P>, rhs: MnaRhs<P>) -> Vec<P> { 
    solve(matrix.get_matrix(), rhs.get_vector())
}
