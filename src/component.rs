
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
    /// Independent voltage source (group2)
    IndependentVoltageSource {
	term_pos: usize,
	term_neg: usize,
	current_index: usize,
	voltage: f64,
    },
}


fn get_current_index(tokens: &mut Vec<&str>, next_free_edge: &mut usize) -> Option<usize> {
    // Check if the last element is a group 2 specifier
    let current_index;
    if tokens.last().unwrap().to_string() == "G2" {
	current_index = Some(*next_free_edge);
	tokens.pop();
	*next_free_edge += 1;
    } else {
	current_index = None;
    }
    current_index
}

impl Component {

    fn new_resistor(mut tokens: Vec<&str>, next_free_current_index: &mut usize) -> Self {

	// Check for current_index
	let current_index = get_current_index(&mut tokens, next_free_current_index);

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

    fn new_independent_voltage_source(tokens: Vec<&str>, next_free_current_index: &mut usize) -> Self {

	if tokens.len() != 3 {
	    panic!("Expected three tokens for independent voltage source")
	}

	let current_index = *next_free_current_index;
	*next_free_current_index += 1;
	
	let term_pos = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_neg = tokens[1].parse().expect("Failed to parse negative terminal");
	let voltage = tokens[2].parse().expect("Failed to parse resistance value");
	
	Self::IndependentVoltageSource {
	    term_pos,
	    term_neg,
	    current_index,
	    voltage,
	}
    }
    
    pub fn new(name: &str, tokens: Vec<&str>, next_free_edge: &mut usize) -> Self {
	match name {
	    "r" => Self::new_resistor(tokens, next_free_edge),
	    "v" => Self::new_independent_voltage_source(tokens, next_free_edge),
	    &_ => todo!("Not yet implemented {name}"),
	}
    }
}
