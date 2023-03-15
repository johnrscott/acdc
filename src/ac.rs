//! AC analysis

use crate::mna::Mna;
use num::Complex;
use std::f64::consts::PI;
use core::ops;

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
	self.mna.add_resistor(term_1, term_2, current_edge, r);
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
	self.mna.add_resistor(term_1, term_2, current_edge, x);
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
	self.mna.add_resistor(term_1, term_2, current_edge, x);
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

struct Impedance {
    term_1: usize,
    term_2: usize,
    current_edge: Option<usize>,
    /// Closure to calculate the impedance at
    /// a particular frequency
    impedance: Box<dyn ops::Fn(f64) -> Complex<f64>>,
}

struct VoltageSource {
    term_pos: usize,
    term_neg: usize,
    current_edge: usize,
    voltage: f64,
}

struct LinearAcSweep {
    f_start: f64,
    f_end: f64,
    num_steps: usize,
    f: Vec<f64>,
    impedances: Vec<Impedance>,
    voltage_sources: Vec<VoltageSource>,
}

impl LinearAcSweep {
    pub fn new(f_start: f64, f_end: f64, num_steps: usize) -> Self {
	Self {
	    f_start,
	    f_end,
	    num_steps,
	    f: (0..num_steps)
		.map(|n| f_start + (n as f64) / (f_end - f_start))
		.collect(),
	    impedances: Vec::new(),
	    voltage_sources: Vec::new(),
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	resistance: f64,
    ) {
	let element = Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Box::new(|_f| Complex::new(resistance, 0.0)),
	};
	self.impedances.push(element);
    }

    pub fn add_capacitor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	capacitance: f64,
    ) {
	let element = Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Box::new(|f| {
		let omega = 2.0 * PI * f;
		let s = Complex::i() * omega;
		1.0 / (s * capacitance)
	    })
	};
	self.impedances.push(element);
    }

    pub fn add_inductor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	inductance: f64,
    ) {
	let element = Impedance {
	    term_1,
	    term_2,
	    current_edge,
	    impedance: Box::new(|f| {
		let omega = 2.0 * PI * f;
		let s = Complex::i() * omega;
		s * inductance
	    })
	};
	self.impedances.push(element);
    }
    
    pub fn add_independent_voltage_source(
    	&mut self,
    	term_pos: usize,
    	term_neg: usize,
    	current_edge: usize,
    	voltage: f64,
    ) {
	let source = VoltageSource {
	    term_pos,
	    term_neg,
	    current_edge,
	    voltage,
	};
	self.voltage_sources.push(source);
    }

    
}
