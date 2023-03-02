use crate::circuit::element::Element;
use crate::circuit::{MnaMatrix, MnaRhs};

pub struct VoltageSource {
    voltage: f64,
    terminal_plus: String,
    terminal_minus: String,
}

impl VoltageSource {
    pub fn new((terminal_plus, terminal_minus): (String, String), voltage: f64)
	   -> Self {
	Self {
	    voltage,
	    terminal_plus,
	    terminal_minus
	}
    }
}

impl std::fmt::Display for VoltageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Voltage Source, V = {} Volt, {} -- {}",
	       self.voltage, self.terminal_plus, self.terminal_minus)
    }
}

impl Element for VoltageSource {
    fn terminal_names(&self) -> Vec<String> {
	vec![self.terminal_plus.to_string(),
	     self.terminal_minus.to_string()]
    }
    fn add_stamp(&self,
		 mna_matrix: &mut MnaMatrix,
		 mna_rhs: &mut MnaRhs,
		 node_indices: Vec<usize>,
		 element_index: usize,
		 group2: bool) {
	println!("Not done! (voltage source add_stamp)");
    }

}
