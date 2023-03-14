use mna::Mna;

mod mna;
mod sparse;

fn main() {
    let mut mna = Mna::new();
    mna.add_resistor(1, 0, None, 50.0);
    mna.add_resistor(2, 1, None, 50.0);
    mna.add_independent_voltage_source(2, 0, 0, 5.0);
    let (voltages, currents) = mna.solve();
    println!("{:?}", voltages);
    println!("{:?}", currents);   
}
