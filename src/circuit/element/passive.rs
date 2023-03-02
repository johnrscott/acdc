use crate::circuit::{MnaMatrix, MnaRhs};
use crate::circuit::element::Element;

pub struct Resistor {
    resistance: f64,
    terminal_1: String,
    terminal_2: String,
}

impl Resistor {
    pub fn new((terminal_1, terminal_2): (String, String), resistance: f64) -> Self {
	Self {
	    resistance,
	    terminal_1,
	    terminal_2
	}
    }
}

impl std::fmt::Display for Resistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resistor, R = {} Ohm, {} -- {}",
	       self.resistance, self.terminal_1, self.terminal_2)
    }
}

impl Element for Resistor {
    fn terminal_names(&self) -> Vec<String> {
	vec![self.terminal_1.to_string(),
	     self.terminal_2.to_string()]
    }
    fn add_stamp(&self,
		 mna_matrix: &mut MnaMatrix,
		 _mna_rhs: &mut MnaRhs,
		 node_indices: Vec<usize>,
		 element_index: usize,
		 group2: bool) {
	let n = node_indices;
	let r = self.resistance;
	if group2 {
	    let e = element_index;
	    mna_matrix.insert_group2(e, n[0],  1.0);
	    mna_matrix.insert_group2(e, n[1], -1.0);
	    mna_matrix.insert_group2(n[0], e,  1.0);
	    mna_matrix.insert_group2(n[1], e, -1.0);	    
	} else {
	    mna_matrix.insert_group1(n[0], n[0],  1.0/r);
	    mna_matrix.insert_group1(n[0], n[1], -1.0/r);
	    mna_matrix.insert_group1(n[1], n[0],  1.0/r);
	    mna_matrix.insert_group1(n[1], n[0], -1.0/r);
	}
    }
}

pub struct Capacitor {
    capacitance: f64,
    terminal_1: String,
    terminal_2: String,
}

impl Capacitor {
    pub fn new((terminal_1, terminal_2): (String, String), capacitance: f64) -> Self {
	Self {
	    capacitance,
	    terminal_1,
	    terminal_2
	}
    }
}

impl std::fmt::Display for Capacitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capacitor, C = {} Farad, {} -- {}",
	       self.capacitance, self.terminal_1, self.terminal_2)
    }
}

impl Element for Capacitor {
    fn terminal_names(&self) -> Vec<String> {
	vec![self.terminal_1.to_string(),
	     self.terminal_2.to_string()]
    }
    fn add_stamp(&self,
		 mna_matrix: &mut MnaMatrix,
		 mna_rhs: &mut MnaRhs,
		 node_indices: Vec<usize>,
		 element_index: usize,
		 group2: bool) {
	println!("Not done! (capacitor add_stamp)");
    }
}
