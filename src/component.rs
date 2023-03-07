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
    /// Current-controlled voltage source (group2)
    CurrentControlledVoltageSource {
        term_pos: usize,
        term_neg: usize,
	ctrl_edge: usize,
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

fn in_group2(tokens: &mut Vec<&str>) -> bool {
    // Check if the last element is a group 2 specifier
    if tokens.last().unwrap().to_string() == "G2" {
	tokens.pop();
	true
    } else {
	false
    }
}

impl Component {
    fn new_resistor(
	name_id: &str,
        mut tokens: Vec<&str>,
        node_map: &mut NodeMap,
    ) -> Self {

	// Check for current_index
	let current_index;
	if in_group2(&mut tokens) {
	    current_index = Some(node_map.allocate_edge(name_id));
	} else {
	    current_index = None;
	}

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
	name_id: &str,
	tokens: Vec<&str>,
        node_map: &mut NodeMap,
    ) -> Self {
        if tokens.len() != 3 {
            panic!("Expected three tokens for independent voltage source")
        }

	let current_index = node_map.allocate_edge(name_id);
	
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
	name_id: &str,
	tokens: Vec<&str>,
	node_map: &mut NodeMap,
    ) -> Self {
	if tokens.len() != 5 {
            panic!("Expected five tokens for VCVS")
        }
	
       	let current_index = node_map.allocate_edge(name_id);

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

    fn new_current_controlled_voltage_source(
	name_id: &str,
	tokens: Vec<&str>,
	node_map: &mut NodeMap,
    ) -> Self {
	if tokens.len() != 5 {
            panic!("Expected four tokens for CCVS")
        }

        let term_pos = node_map.allocate_index(tokens[0]);
        let term_neg = node_map.allocate_index(tokens[1]);
        let ctrl_edge = node_map.allocate_edge(tokens[2]);
        let voltage_scale = tokens[4].parse()
	    .expect("Failed to parse resistance value");

	let current_index = node_map.allocate_edge(name_id);
	
        Self::CurrentControlledVoltageSource {
            term_pos,
            term_neg,
	    ctrl_edge,
	    current_index,
            voltage_scale,
        }
    }
    
    fn new_independent_current_source(
	name_id: &str,
        mut tokens: Vec<&str>,
        node_map: &mut NodeMap,
    ) -> Self {
	// Check for current_index
	let current_index;
	if in_group2(&mut tokens) {
	    current_index = Some(node_map.allocate_edge(name_id));
	} else {
	    current_index = None;
	}
	
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
	name_id: &str,
        tokens: Vec<&str>,
        node_map: &mut NodeMap,
    ) -> Self {
        match name {
            "r" => Self::new_resistor(name_id, tokens, node_map),
            "v" => Self::new_independent_voltage_source(name_id, tokens, node_map),
            "e" => Self::new_voltage_controlled_voltage_source(name_id, tokens, node_map),
	    "i" => Self::new_independent_current_source(name_id, tokens, node_map),
            &_ => todo!("Not yet implemented component"),
        }
    }

    /// Return the current index, if this element has a current
    pub fn current_index(&self) -> Option<usize> {
        match self {
            Self::IndependentVoltageSource { current_index, .. } => Some(*current_index),
	    Self::VoltageControlledVoltageSource { current_index, .. } => Some(*current_index),
	    Self::CurrentControlledVoltageSource { current_index, .. } => Some(*current_index),
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
        Component::CurrentControlledVoltageSource {
            term_pos,
            term_neg,
	    ctrl_edge,
            voltage_scale,
            ..
        } => print!("{}(+) --- V({voltage_scale} x U V) ---- {}  <--- I({})",
		      node_map.get_node_name(*term_pos),
		      node_map.get_node_name(*term_neg),
		      node_map.get_edge_name(*ctrl_edge),
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
