use std::cmp;
use csuperlu::sparse_matrix::SparseMat;
use crate::sparse::{plus_equals, concat_horizontal, concat_vertical};

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
pub struct MnaMatrix {
    /// The number of rows in the top matrices
    num_voltage_nodes: usize,
    /// The number of rows in the bottom matrices
    num_current_edges: usize,
    top_left: SparseMat<f64>,
    top_right: SparseMat<f64>,
    bottom_left: SparseMat<f64>,
    bottom_right: SparseMat<f64>,
}

impl MnaMatrix {
    pub fn new() -> Self {
        Self {
            num_voltage_nodes: 0,
            num_current_edges: 0,
            top_left: SparseMat::empty(),
            top_right: SparseMat::empty(),
            bottom_left: SparseMat::empty(),
            bottom_right: SparseMat::empty(),
        }
    }

    pub fn num_voltage_nodes(&self) -> usize {
        self.num_voltage_nodes
    }

    pub fn num_current_edges(&self) -> usize {
        self.num_current_edges
    }

    pub fn get_matrix(mut self) -> SparseMat<f64> {
        self.top_left
            .resize(self.num_voltage_nodes, self.num_voltage_nodes);
        self.bottom_right
            .resize(self.num_current_edges, self.num_current_edges);
        self.top_right
            .resize(self.num_voltage_nodes, self.num_current_edges);
        self.bottom_left
            .resize(self.num_current_edges, self.num_voltage_nodes);

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
        &mut self,
        n1: usize,
        n2: usize,
        e: usize,
        x1: f64,
        x2: f64,
        y: f64,
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

    /*
    /// Same as symmetric version, but only adds values to the
    /// right-hand portion of the matrix (top and bottom)
    pub fn add_unsymmetric_right_group2(
        &mut self,
        n1: usize,
        n2: usize,
        e: usize,
        x1: f64,
        x2: f64,
        y: f64,
    ) {
        if n1 == n2 {
            panic!("Cannot set unsymmetric group (right) 2 where n1 == n2");
        }
        self.update_num_voltage_nodes(n1);
        self.update_num_voltage_nodes(n2);
        self.update_num_current_edges(e);
        plus_equals(&mut self.bottom_right, e, e, y);
        if n1 != 0 {
            plus_equals(&mut self.top_right, n1 - 1, e, x1);
        }
        if n2 != 0 {
            plus_equals(&mut self.top_right, n2 - 1, e, x2);
        }
    }
     */

    /*
    /// Same as symmetric version, but only adds values to the
    /// bottom portion of the matrix (left and right)
    pub fn add_unsymmetric_bottom_group2(
        &mut self,
        n1: usize,
        n2: usize,
        e: usize,
        x1: f64,
        x2: f64,
        y: f64,
    ) {
        if n1 == n2 {
            panic!("Cannot set unsymmetric group (bottom) 2 where n1 == n2");
        }
        self.update_num_voltage_nodes(n1);
        self.update_num_voltage_nodes(n2);
        self.update_num_current_edges(e);
        plus_equals(&mut self.bottom_right, e, e, y);
        if n1 != 0 {
            plus_equals(&mut self.bottom_left, e, n1 - 1, x1);
        }
        if n2 != 0 {
            plus_equals(&mut self.bottom_left, e, n2 - 1, x2);
        }
    }
     */

    /*
    /// Add a single value in the group2 (current-current, bottom-right) portion
    /// of the matrix
    pub fn add_group2_value(
        &mut self,
        e1: usize,
        e2: usize,
        y: f64,
    ) {
        self.update_num_current_edges(e1);
        self.update_num_current_edges(e2);
        plus_equals(&mut self.bottom_right, e1, e2, y);
    }
     */
}
