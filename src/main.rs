use crate::dc::DcAnalysis;


mod mna;
mod dc;
mod sparse;

fn main() {

    let mut dc = DcAnalysis::new();
    dc.add_resistor(1, 0, None, 50.0);
    dc.add_resistor(2, 1, None, 50.0);
    dc.add_independent_voltage_source(2, 0, 0, 5.0);
    let (voltages, currents) = dc.solve();
    println!("{:?}", voltages);
    println!("{:?}", currents);   
}
