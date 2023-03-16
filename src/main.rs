use dc::LinearDcAnalysis;


mod mna;
mod dc;
mod ac;
mod sparse;
mod node_map;

fn main() {

    let mut dc = LinearDcAnalysis::new();

    // Voltage divider
    dc.add_resistor("vcc", "v_out", None, 4.7);
    dc.add_resistor("v_out", "gnd", None, 4.7);
    dc.add_independent_voltage_source("vcc", "gnd", "v1", 5.0);

    let (voltages, currents) = dc.solve();
    println!("Voltages: {:?}", voltages);
    println!("Currents: {:?}", currents);
}
