//! DC analysis

use crate::{mna::Mna, node_map::NodeMap};
use csuperlu::c::value_type::ValueType;
use num;

pub struct LinearDcAnalysis<P: ValueType + num::Float> {
    node_map: NodeMap,
    mna: Mna<P>,
}

impl<P: ValueType + num::Float> LinearDcAnalysis<P> {
    pub fn new() -> Self {
	Self {
	    node_map: NodeMap::new(),
	    mna: Mna::new(),
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: &str,
	term_2: &str,
	current_edge: Option<usize>,
	resistor: P,
    ) {
	let term_1 = self.node_map.node_index(term_1);
	let term_2 = self.node_map.node_index(term_2);
	self.mna.add_impedance(term_1, term_2, current_edge, resistor);
    }

    pub fn add_independent_voltage_source(
	&mut self,
	term_pos: &str,
	term_neg: &str,
	current_edge: &str,
	voltage: P,
    ) {
	let term_pos = self.node_map.node_index(term_pos);
	let term_neg = self.node_map.node_index(term_neg);
	let current_edge = self.node_map.edge_index(current_edge);
	self.mna.add_independent_voltage_source(term_pos, term_neg, current_edge, voltage);
    }

    pub fn solve(self) -> (Vec<P>, Vec<P>) {
	self.mna.solve()
    }
    
}
