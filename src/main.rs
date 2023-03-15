use crate::ac::LinearAcAnalysis;

mod mna;
mod dc;
mod ac;
mod sparse;

fn main() {
    let mut ac = LinearAcAnalysis::new(2.0 * std::f64::consts::PI * 100.0);
    ac.add_capacitor(1, 0, None, 1e-9);
    ac.add_independent_voltage_source(1, 0, 0, 5.0);
    let (voltages, currents) = ac.solve();
    println!("{:?}", voltages);
    println!("{:?}", currents);   
}
