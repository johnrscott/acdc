use csuperlu::sparse_matrix::SparseMat;

/// Modified nodal analysis right-hand side
///
/// The right-hand side for modified nodal analysis is
///
/// | -A1 s1 |
/// |        |
/// |   s2   |
///
pub struct MnaRhs {
    top: SparseMat<f64>,
    bottom: SparseMat<f64>,
}

impl MnaRhs {
    pub fn new() -> Self {
        Self {
            top: SparseMat::empty(),
            bottom: SparseMat::empty(),
        }
    }

    pub fn get_vector(self, num_voltage_nodes: usize, num_current_edges: usize) -> Vec<f64> {
        let mut out = vec![0.0; num_voltage_nodes + num_current_edges];
        for ((row, _), value) in self.top.non_zero_vals().iter() {
            out[*row] = *value;
        }
        for ((row, _), value) in self.bottom.non_zero_vals().iter() {
            out[num_voltage_nodes + *row] = *value;
        }
        out
    }

    /*
    /// Add a RHS element in the group 1 matrix.
    pub fn add_rhs_group1(&mut self, n: usize, x: f64) {
	if n != 0 {
            self.top.insert_unbounded(n - 1, 1, x);
	}
    }
     */
    
    /// Add a RHS element in the group 2 matrix
    pub fn add_rhs_group2(&mut self, e: usize, x: f64) {
        self.bottom.insert_unbounded(e, 1, x);
    }
}
