use csuperlu::c::value_type::ValueType;

use std::ops;

use crate::sparse::solve;

use self::{mna_matrix::MnaMatrix, mna_rhs::MnaRhs};

mod mna_matrix;
mod mna_rhs;

pub struct Mna<P: ValueType + ops::Neg<Output=P>> {
    matrix: MnaMatrix<P>,
    rhs: MnaRhs<P>,
}

impl<P: ValueType + ops::Neg<Output=P>> Mna<P> {
    pub fn new() -> Self {
        Self {
            matrix: MnaMatrix::new(),
            rhs: MnaRhs::new(),
        }
    }

    pub fn add_impedance(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	impedance: P,
    ) {
	let z = impedance;
        match current_edge {
            Some(e) => self
                .matrix
                .add_symmetric_group2(term_1, term_2, e, P::one(), -P::one(), -z),
            None => self
                .matrix
                .add_symmetric_group1(term_1, term_2, P::one() / z, -P::one() / z),
        };
    }

    pub fn add_independent_voltage_source(
	&mut self,
	term_pos: usize,
	term_neg: usize,
	current_edge: usize,
	voltage: P,
    ) {
	let v = voltage;
        self.matrix.add_symmetric_group2(
            term_pos,
            term_neg,
            current_edge,
            P::one(),
            -P::one(),
            P::zero(),
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
    pub fn solve(self) -> (Vec<P>, Vec<P>) {
        let num_voltage_nodes = self.matrix.num_voltage_nodes();
        let num_current_edges = self.matrix.num_current_edges();
        let matrix = self.matrix.get_matrix();

	matrix.print_structure(num_voltage_nodes);
	
	let rhs = self.rhs.get_vector(num_voltage_nodes, num_current_edges);

	let mut solution = solve(matrix, rhs);
	let currents: Vec<_> = solution
	    .drain(num_voltage_nodes..)
	    .collect();
	// Solution now contains the voltages
	(solution, currents)
    }
}
