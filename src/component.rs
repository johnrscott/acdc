use std::fmt;

use crate::NodeMap;

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
    /// Voltage-controlled voltage source (group2)
    VoltageControlledVoltageSource {
        term_pos: usize,
        term_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        current_index: usize,
        voltage_scale: f64,
    },
    /// Independent voltage source (group1 or group2)
    IndependentCurrentSource {
        term_pos: usize,
        term_neg: usize,
        current_index: Option<usize>,
        current: f64,
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
    fn new_resistor(
        mut tokens: Vec<&str>,
        next_free_current_index: &mut usize,
        node_map: &mut NodeMap,
    ) -> Self {
        // Check for current_index
        let current_index = get_current_index(&mut tokens, next_free_current_index);

        if tokens.len() != 3 {
            panic!("Expected three tokens for Resistor")
        }

        let term_1 = node_map.allocate_index(tokens[0]);
        let term_2 = node_map.allocate_index(tokens[1]);
        let resistance = tokens[2].parse().expect("Failed to parse resistance value");

        Self::Resistor {
            term_1,
            term_2,
            current_index,
            resistance,
        }
    }

    fn new_independent_voltage_source(
        tokens: Vec<&str>,
        next_free_current_index: &mut usize,
        node_map: &mut NodeMap,
    ) -> Self {
        if tokens.len() != 3 {
            panic!("Expected three tokens for independent voltage source")
        }

        let current_index = *next_free_current_index;
        *next_free_current_index += 1;

        let term_pos = node_map.allocate_index(tokens[0]);
        let term_neg = node_map.allocate_index(tokens[1]);
        let voltage = tokens[2].parse().expect("Failed to parse resistance value");

        Self::IndependentVoltageSource {
            term_pos,
            term_neg,
            current_index,
            voltage,
        }
    }
    
    fn new_voltage_controlled_voltage_source(
	tokens: Vec<&str>,
	next_free_current_index: &mut usize,
	node_map: &mut NodeMap,
    ) -> Self {
	if tokens.len() != 5 {
            panic!("Expected five tokens for VCVS")
        }
	
        let current_index = *next_free_current_index;
        *next_free_current_index += 1;

        let term_pos = node_map.allocate_index(tokens[0]);
        let term_neg = node_map.allocate_index(tokens[1]);
        let ctrl_pos = node_map.allocate_index(tokens[2]);
        let ctrl_neg = node_map.allocate_index(tokens[3]);
        let voltage_scale = tokens[4].parse()
	    .expect("Failed to parse resistance value");

        Self::VoltageControlledVoltageSource {
            term_pos,
            term_neg,
	    ctrl_pos,
	    ctrl_neg,
	    current_index,
            voltage_scale,
        }
    }
    
    fn new_independent_current_source(
        mut tokens: Vec<&str>,
        next_free_current_index: &mut usize,
        node_map: &mut NodeMap,
    ) -> Self {
        // Check for current_index
        let current_index = get_current_index(&mut tokens, next_free_current_index);

        if tokens.len() != 3 {
            panic!("Expected three tokens for independent current source")
        }

        let term_pos = node_map.allocate_index(tokens[0]);
        let term_neg = node_map.allocate_index(tokens[1]);
        let current = tokens[2].parse().expect("Failed to parse current value");

        Self::IndependentCurrentSource {
            term_pos,
            term_neg,
            current_index,
            current,
        }
    }

    
    pub fn new(
        name: &str,
        tokens: Vec<&str>,
        next_free_edge: &mut usize,
        node_map: &mut NodeMap,
    ) -> Self {
        match name {
            "r" => Self::new_resistor(tokens, next_free_edge, node_map),
            "v" => Self::new_independent_voltage_source(tokens, next_free_edge, node_map),
            "e" => Self::new_voltage_controlled_voltage_source(tokens, next_free_edge, node_map),
	    "i" => Self::new_independent_current_source(tokens, next_free_edge, node_map),
            &_ => todo!("Not yet implemented component"),
        }
    }

    /// Return the current index, if this element has a current
    pub fn current_index(&self) -> Option<usize> {
        match self {
            Self::IndependentVoltageSource { current_index, .. } => Some(*current_index),
	    Self::VoltageControlledVoltageSource { current_index, .. } => Some(*current_index),
	    Self::Resistor { current_index, .. } => *current_index,
	    Self::IndependentCurrentSource { current_index, .. } => *current_index,
        }
    }
}


pub fn print_component(component: &Component, node_map: &NodeMap) {
    match component {
        Component::Resistor {
            term_1,
            term_2,
            resistance,
            ..
        } => print!("{} ---- R({resistance} Ohm) ---- {}",
		     node_map.get_node_name(*term_1),
		     node_map.get_node_name(*term_2)
	    ),
        Component::IndependentVoltageSource {
            term_pos,
            term_neg,
            voltage,
            ..
        } => print!("{}(+) --- V({voltage} V) ---- {}",
		      node_map.get_node_name(*term_pos),
		      node_map.get_node_name(*term_neg)
	),
        Component::VoltageControlledVoltageSource {
            term_pos,
            term_neg,
	    ctrl_pos,
	    ctrl_neg,
            voltage_scale,
            ..
        } => print!("{}(+) --- V({voltage_scale} x U V) ---- {}  <--- {}(+) --- V(U V) ---- {}",
		      node_map.get_node_name(*term_pos),
		      node_map.get_node_name(*term_neg),
		      node_map.get_node_name(*ctrl_pos),
		      node_map.get_node_name(*ctrl_neg)
	),
        Component::IndependentCurrentSource {
            term_pos,
            term_neg,
            current,
            ..
        } => print!("{}(+) --- I({current} A) ---- {}",
		      node_map.get_node_name(*term_pos),
		      node_map.get_node_name(*term_neg),
	),
    }
}
