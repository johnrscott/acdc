//! AC analysis

use crate::mna::Mna;
use num::Complex;
use std::f64::consts::PI;

pub struct LinearAcAnalysis {
    omega: f64,
    mna: Mna<Complex<f64>>,
}

impl LinearAcAnalysis {
    /// New AC analysis at radial frequency omega
    pub fn new(omega: f64) -> Self {
	Self {
	    omega,
	    mna: Mna::new(),
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	resistance: f64,
    ) {
	let r = Complex::new(resistance, 0.0);
	self.mna.add_impedance(term_1, term_2, current_edge, r);
    }

    pub fn add_capacitor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	capacitance: f64,
    ) {
	let s = Complex::i() * self.omega;
	let x = 1.0 / (s * capacitance);
	self.mna.add_impedance(term_1, term_2, current_edge, x);
    }

    pub fn add_inductor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	inductance: f64,
    ) {
	let s = Complex::i() * self.omega;
	let x = s * inductance;
	self.mna.add_impedance(term_1, term_2, current_edge, x);
    }
    
    pub fn add_independent_voltage_source(
    	&mut self,
    	term_pos: usize,
    	term_neg: usize,
    	current_edge: usize,
    	voltage: f64,
    ) {
	let v = Complex::new(voltage, 0.0);
    	self.mna.add_independent_voltage_source(term_pos, term_neg, current_edge, v);
    }

    pub fn solve(self) -> (Vec<Complex<f64>>, Vec<Complex<f64>>) {
	self.mna.solve()
    }
    
}

enum Impedance {
    Resistor(f64),
    Capacitor(f64),
    Inductor(f64),
}

impl Impedance {
    fn impedance(&self, freq_hz: f64) -> Complex<f64> {
	match self {
	    Impedance::Resistor(r) => Complex::new(*r, 0.0),
	    Impedance::Capacitor(c) => {
		let omega = 2.0 * PI * freq_hz;
		let s = Complex::i() * omega;		
		1.0 / (s * c)
	    },
	    Impedance::Inductor(l) => {
		let omega = 2.0 * PI * freq_hz;
		let s = Complex::i() * omega;		
		s * l		
	    }
	}
    }
}

enum Element {
    Impedance {
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	/// Closure to calculate the impedance at
	/// a particular frequency
	impedance: Impedance,
    },
    VoltageSource {
	term_pos: usize,
	term_neg: usize,
	current_edge: usize,
	voltage: f64,
    },
}

pub struct LinearAcSweep {
    f_start: f64,
    f_end: f64,
    num_steps: usize,
    f: Vec<f64>,
    elements: Vec<Element>,
}

impl LinearAcSweep {
    pub fn new(f_start: f64, f_end: f64, num_steps: usize) -> Self {
	Self {
	    f_start,
	    f_end,
	    num_steps,
	    f: (0..num_steps)
		.map(|n| f_start + (n as f64) * (f_end - f_start) / num_steps as f64)
		.collect(),
	    elements: Vec::new(),
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	resistance: f64,
    ) {
	let element = Element::Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Impedance::Resistor(resistance),
	};
	self.elements.push(element);
    }

    pub fn add_capacitor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	capacitance: f64,
    ) {
	let element = Element::Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Impedance::Capacitor(capacitance),
	};
	self.elements.push(element);
    }

    pub fn add_inductor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	inductance: f64,
    ) {
	let element = Element::Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Impedance::Inductor(inductance),
	};
	self.elements.push(element);
    }
    
    pub fn add_independent_voltage_source(
    	&mut self,
    	term_pos: usize,
    	term_neg: usize,
    	current_edge: usize,
    	voltage: f64,
    ) {
	let source = Element::VoltageSource {
	    term_pos,
	    term_neg,
	    current_edge,
	    voltage,
	};
	self.elements.push(source);
    }

    pub fn solve(&self) -> (Vec<f64>, Vec<Vec<Complex<f64>>>, Vec<Vec<Complex<f64>>>) {

	let mut voltages_with_frequency = Vec::new();
	let mut currents_with_frequency = Vec::new();
	
	for freq_hz in self.f.iter() {

	    // Make new modal analysis matrix
	    let mut mna = Mna::new();

	    // Add all the elements
	    for elem in self.elements.iter() {
		match elem {
		    Element::Impedance {
			term_1,
			term_2,
			current_edge,
			impedance
		    } => {
			mna.add_impedance(*term_1, *term_2, *current_edge,
					 impedance.impedance(*freq_hz));
		    },
		    Element::VoltageSource {
			term_pos,
			term_neg,
			current_edge,
			voltage
		    } => {
    			mna.add_independent_voltage_source(*term_pos, *term_neg, *current_edge,
							   voltage.into());
		    }
		}
	    }

	    // Solve the system at this frequency
	    let (voltages, currents) = mna.solve();

	    voltages_with_frequency.push(voltages);
	    currents_with_frequency.push(currents);
	}

	
	
	// Convert to vectors of voltage with frequency at each node
	let mut v = Vec::new();
	let num_voltage_nodes = voltages_with_frequency[0].len();

	// Loop over voltages
	for n in 0..num_voltage_nodes {
	    let mut v_at_one_frequency = Vec::new();
	    // Loop over frequencies
	    for m in 0..self.f.len() {
		v_at_one_frequency.push(voltages_with_frequency[m][n])
	    }
	    v.push(v_at_one_frequency);
	}
	
	let mut i = Vec::new();
	let num_current_nodes = currents_with_frequency[0].len();

	// Loop over currents
	for n in 0..num_current_nodes {
	    let mut i_at_one_frequency = Vec::new();
	    // Loop over frequencies
	    for m in 0..self.f.len() {
		i_at_one_frequency.push(currents_with_frequency[m][n])
	    }
	    i.push(i_at_one_frequency);
	}

	
	    
	(self.f.to_vec(), v, i)
    }
}
