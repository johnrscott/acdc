//! DC analysis

use crate::mna::Mna;
use csuperlu::c::value_type::ValueType;
use num;

pub struct LinearDcAnalysis<P: ValueType + num::Float> {
    mna: Mna<P>,
}

impl<P: ValueType + num::Float> LinearDcAnalysis<P> {
    pub fn new() -> Self {
	Self {
	    mna: Mna::new(),
	}
    }

    pub fn add_impedance(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	impedance: P,
    ) {
	self.mna.add_impedance(term_1, term_2, current_edge, impedance);
    }

    pub fn add_independent_voltage_source(
	&mut self,
	term_pos: usize,
	term_neg: usize,
	current_edge: usize,
	voltage: P,
    ) {
	self.mna.add_independent_voltage_source(term_pos, term_neg, current_edge, voltage);
    }

    pub fn solve(self) -> (Vec<P>, Vec<P>) {
	self.mna.solve()
    }
    
}
