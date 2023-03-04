
/// Component type
///
/// Components are either in group 1 (their currents are eliminated),
/// or group 2 (currents eliminated).
///
/// The following elements are always in group 2:
/// - Voltage sources (independent or controlled)
/// - Vontrol element for a current-controlled source
/// - Element whose current is a user-requested output.
///
/// The following elements can be in group 1 or group 2:
/// - Resistors
/// - Independent current source
/// - Voltage-controlled current source
/// - Current-controlled current source
/// - Control element for a voltage-controlled current source
///
#[derive(Debug)]
pub enum Component {
    /// Independent voltage source (group 2)
    VoltageSource { term_pos: usize, term_neg: usize, voltage: f64 },
    /// Independent current source (group 1 or group 2)
    CurrentSource { term_pos: usize, term_neg: usize, current: f64, group2: bool },
    /// Fixed resistor (group 1 or group 2)
    Resistor { edge: usize, term_1: usize, term_2: usize, resistance: f64, group2: bool },
    /// Capacitor
    Capacitor { term_1: usize, term_2: usize, capacitance: f64 },
    /// Inductor
    Inductor { term_1: usize, term_2: usize, inductance: f64 },
    Diode { term_anode: usize, term_cathode: usize },
    BjtNpn { term_collector: usize, term_base: usize, term_emitter: f64 },
    BjtPnp { term_emitter: usize, term_base: usize, term_collector: f64 },
    NMos { term_drain: usize, term_gate: usize, term_source: f64 },
    PMos { term_source: usize, term_gate: usize, term_drain: f64 },
}

impl Component {

    pub fn group2(&self) -> bool {
	match self {
	    Self::VoltageSource{..} => true,
	    Self::CurrentSource{group2, ..} => *group2,
	    Self::Resistor{group2, ..} => *group2,
	    _ => false
	}
    }
    
    fn new_voltage_source(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for VoltageSource")
	}
	let term_pos = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_neg = tokens[1].parse().expect("Failed to parse negative terminal");
	let voltage = tokens[2].parse().expect("Failed to parse voltage value");
	Self::VoltageSource {
	    term_pos,
	    term_neg,
	    voltage,
	}
    }
    
    fn new_current_source(tokens: Vec<&str>, group2: bool) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for CurrentSource")
	}
	let term_pos = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_neg = tokens[1].parse().expect("Failed to parse negative terminal");
	let current = tokens[2].parse().expect("Failed to parse current value");
	Self::CurrentSource {
	    term_pos,
	    term_neg,
	    current,
	    group2,
	}
    }

    fn new_resistor(tokens: Vec<&str>, group2: bool, edge: usize) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for Resistor")
	}
	let term_1 = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_2 = tokens[1].parse().expect("Failed to parse negative terminal");
	let resistance = tokens[2].parse().expect("Failed to parse resistance value");
	
	Self::Resistor {
	    edge,
	    term_1,
	    term_2,
	    resistance,
	    group2,
	}
    }

    fn new_capacitor(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for Capacitor")
	}
	let term_1 = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_2 = tokens[1].parse().expect("Failed to parse negative terminal");
	let capacitance = tokens[2].parse().expect("Failed to parse capacitance value");
	Self::Capacitor {
	    term_1,
	    term_2,
	    capacitance,
	}
    }
    
    fn new_inductor(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for Inductor")
	}
	let term_1 = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_2 = tokens[1].parse().expect("Failed to parse negative terminal");
	let inductance = tokens[2].parse().expect("Failed to parse inductance value");
	Self::Inductor {
	    term_1,
	    term_2,
	    inductance,
	}
    }

    fn new_diode(tokens: Vec<&str>) -> Self {
	if tokens.len() != 2 {
	    panic!("Expected two tokens for Diode")
	}
	let term_anode = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_cathode = tokens[1].parse().expect("Failed to parse negative terminal");
	Self::Diode {
	    term_anode,
	    term_cathode,
	}
    }

    fn new_bjt_npn(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for NPN BJT")
	}
	let term_collector = tokens[0].parse().expect("Failed to parse collector terminal");
	let term_base = tokens[1].parse().expect("Failed to parse base terminal");
	let term_emitter = tokens[1].parse().expect("Failed to parse emitter terminal");
	Self::BjtNpn {
	    term_collector,
	    term_base,
	    term_emitter,
	}
    }
    
    fn new_bjt_pnp(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for PNP BJT")
	}
	let term_emitter = tokens[0].parse().expect("Failed to parse emitter terminal");
	let term_base = tokens[1].parse().expect("Failed to parse base terminal");
	let term_collector = tokens[1].parse().expect("Failed to parse collector terminal");
	Self::BjtPnp {
	    term_emitter,
	    term_base,
	    term_collector,
	}
    }
    
    fn new_nmos(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for NMOS")
	}
	let term_drain = tokens[0].parse().expect("Failed to parse drain terminal");
	let term_gate = tokens[1].parse().expect("Failed to parse gate terminal");
	let term_source = tokens[1].parse().expect("Failed to parse source terminal");
	Self::NMos {
	    term_drain,
	    term_gate,
	    term_source,
	}
    }

    fn new_pmos(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for PMOS")
	}
	let term_source = tokens[0].parse().expect("Failed to parse source terminal");
	let term_gate = tokens[1].parse().expect("Failed to parse gate terminal");
	let term_drain = tokens[1].parse().expect("Failed to parse drain terminal");
	Self::PMos {
	    term_source,
	    term_gate,
	    term_drain,
	}
    }
    
    pub fn new(name: &str, tokens: Vec<&str>, group2: bool, edge: usize) -> Self {
	match name {
	    "v" => Self::new_voltage_source(tokens),
	    "i" => Self::new_current_source(tokens, group2),
	    "r" => Self::new_resistor(tokens, group2, edge),
	    "c" => Self::new_capacitor(tokens),
	    "l" => Self::new_inductor(tokens),
	    "d" => Self::new_diode(tokens),
	    "qn" => Self::new_bjt_npn(tokens),
	    "qp" => Self::new_bjt_pnp(tokens),
	    "mn" => Self::new_nmos(tokens),
	    "mp" => Self::new_pmos(tokens),
	    &_ => panic!("Found unexpected name {name}"),
	}
    }
}
