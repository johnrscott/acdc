
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
    /// Fixed resistor (group1 or group2)
    Resistor {
	term_1: usize,
	term_2: usize,
	current_index: Option<usize>,
	resistance: f64,
    },
}

impl Component {

    fn new_resistor(tokens: Vec<&str>, current_index: Option<usize>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for Resistor")
	}
	let term_1 = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_2 = tokens[1].parse().expect("Failed to parse negative terminal");
	let resistance = tokens[2].parse().expect("Failed to parse resistance value");
	
	Self::Resistor {
	    term_1,
	    term_2,
	    current_index,
	    resistance,
	}
    }

    pub fn new(name: &str, tokens: Vec<&str>, current_index: Option<usize>) -> Self {
	match name {
	    "r" => Self::new_resistor(tokens, current_index),
	    &_ => todo!("Not yet implemented {name}"),
	}
    }
}
