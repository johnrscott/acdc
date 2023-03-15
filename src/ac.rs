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
	self.mna.add_resistor(term_1, term_2, current_edge, r);
    }

    pub fn add_capacitor(
	&mut self,
	term_1: usize,
	term_2: usize,
	current_edge: Option<usize>,
	capacitance: f64,
    ) {
	let s = Complex::i() * 2.0 * PI * self.omega;
	let r = 1.0 / (s * capacitance);
	self.mna.add_resistor(term_1, term_2, current_edge, r);
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
