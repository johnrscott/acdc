use crate::ac::{LinearAcAnalysis, LinearAcSweep};

mod mna;
mod dc;
mod ac;
mod sparse;

fn main() {

    let mut ac_sweep = LinearAcSweep::new(1.0, 2.0, 3);
    ac_sweep.add_capacitor(1, 0, None, 1e-9);
    ac_sweep.add_inductor(1, 0, None, 10e-9);
    ac_sweep.add_resistor(2, 1, None, 10.0);
    ac_sweep.add_independent_voltage_source(2, 0, 0, 5.0);

    let (freq, voltages_with_freq, currents_with_freq) = ac_sweep.solve();
    
    println!("{:?}", freq);
    println!("{:?}", voltages_with_freq);
    println!("{:?}", currents_with_freq);   
}
