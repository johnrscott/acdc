use libesim::dc;
use pyo3::prelude::*;

#[pyclass]
struct LinearDcAnalysis {
    dc: dc::LinearDcAnalysis<f64>,
}

#[pymethods]
impl LinearDcAnalysis {
    #[new]
    fn new() -> Self {
        Self {
	    dc: dc::LinearDcAnalysis::new()
	}
    }

    pub fn add_resistor(
	&mut self,
	term_1: usize,
	term_2: usize,
	resistance: f64,
	current_edge: Option<usize>,
    ) {
	self.dc.add_resistor(term_1, term_2, current_edge, resistance);
    }

    pub fn add_independent_voltage_source(
	&mut self,
	term_pos: usize,
	term_neg: usize,
	voltage: f64,
	current_edge: usize,
    ) {
	self.dc.add_independent_voltage_source(term_pos, term_neg, current_edge, voltage);
    }

    pub fn solve(self) -> (Vec<f64>, Vec<f64>) {
	self.dc.solve()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyesim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearDcAnalysis>()?;
    Ok(())
}
