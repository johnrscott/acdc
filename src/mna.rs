use crate::sparse::solve;

use self::{mna_matrix::MnaMatrix, mna_rhs::MnaRhs};

mod mna_matrix;
mod mna_rhs;

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

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	resistance: f64,
    ) {
	let r = resistance;
        match current_edge {
            Some(e) => self
                .matrix
                .add_symmetric_group2(term_1, term_2, e, 1.0, -1.0, -r),
            None => self
                .matrix
                .add_symmetric_group1(term_1, term_2, 1.0 / r, -1.0 / r),
        };
    }

    pub fn add_independent_voltage_source(
	&mut self,
	term_pos: usize,
	term_neg: usize,
	current_edge: usize,
	voltage: f64,
    ) {
	let v = voltage;
        self.matrix.add_symmetric_group2(
            term_pos,
            term_neg,
            current_edge,
            1.0,
            -1.0,
            0.0,
        );
        self.rhs.add_rhs_group2(current_edge, v);
    }
    
    /* Unclean!
    pub fn add_element_stamp(&mut self, component: &Component) {
        match component {
            Component::VoltageControlledVoltageSource {
                term_pos,
                term_neg,
		ctrl_pos,
		ctrl_neg,
                current_index,
                voltage_scale: k,
            } => {
                self.matrix.add_symmetric_group2(
                    *term_pos,
                    *term_neg,
                    *current_index,
                    1.0,
                    -1.0,
                    0.0,
                );
		self.matrix.add_unsymmetric_bottom_group2(
		    *ctrl_pos,
		    *ctrl_neg,
		    *current_index,
		    -*k,
		    *k,
		    0.0,
		);
            },
            Component::CurrentControlledVoltageSource {
                term_pos,
                term_neg,
		ctrl_edge,
                current_index,
                voltage_scale: k,
            } => {
                self.matrix.add_symmetric_group2(
                    *term_pos,
                    *term_neg,
                    *current_index,
                    1.0,
                    -1.0,
                    0.0,
                );
		self.matrix.add_group2_value(*current_index, *ctrl_edge, -*k);
            },
	    Component::IndependentCurrentSource {
                term_pos,
                term_neg,
                current_index,
                current: i
            } => {
                match current_index {
                    Some(edge) => {
			self.matrix.add_unsymmetric_right_group2(
			    *term_pos, *term_neg, *edge,
			    1.0, -1.0, 1.0);
			self.rhs.add_rhs_group2(*edge, *i);
		    },
                    None => {
			self.rhs.add_rhs_group1(*term_pos, -*i);
			self.rhs.add_rhs_group1(*term_neg, *i);
		    }
                }
            },
            _ => todo!("Not currently implemented"),
        }
    }
     */

    /// Returns node voltages, edge currents
    pub fn solve(self) -> (Vec<f64>, Vec<f64>) {
        let num_voltage_nodes = self.matrix.num_voltage_nodes();
        let num_current_edges = self.matrix.num_current_edges();
        let matrix = self.matrix.get_matrix();
        let rhs = self.rhs.get_vector(num_voltage_nodes, num_current_edges);

	let mut solution = solve(matrix, rhs);
	let currents: Vec<_> = solution.drain(num_voltage_nodes..)
	    .collect();
	// Solution now contains the voltages
	(solution, currents)
    }
}
