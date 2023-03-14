use libesim::dc::LinearDcAnalysis;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn test() {
    let mut dc = LinearDcAnalysis::new();
    dc.add_resistor(1, 0, None, 50.0);
    dc.add_resistor(2, 1, None, 50.0);
    dc.add_independent_voltage_source(2, 0, 0, 5.0);
    let (voltages, currents) = dc.solve();
    println!("{:?}", voltages);
    println!("{:?}", currents);   

}

/// A Python module implemented in Rust.
#[pymodule]
fn pyesim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
