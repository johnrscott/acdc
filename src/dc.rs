//! DC analysis

use crate::mna::Mna;
use csuperlu::c::value_type::ValueType;
use num;

pub struct DcAnalysis<P: ValueType + num::Float> {
    mna: Mna<P>,
}

impl<P: ValueType + num::Float> DcAnalysis<P> {
    pub fn new() -> Self {
	Self {
	    mna: Mna::new(),
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	resistance: P,
    ) {
	self.mna.add_resistor(term_1, term_2, current_edge, resistance);
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
