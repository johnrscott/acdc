use std::{fs::File, io::BufReader, io::{BufRead, Lines}, str::FromStr};

use regex::Regex;

#[derive(Debug)]
enum Component {
    VoltageSource { term_pos: usize, term_neg: usize, voltage: f64 },
    CurrentSource { term_pos: usize, term_neg: usize, current: f64 },
    Resistor { term_1: usize, term_2: usize, resistance: f64 },
    Capacitor { term_1: usize, term_2: usize, capacitance: f64 },
    Inductor { term_1: usize, term_2: usize, inductance: f64 },
    Diode { term_anode: usize, term_cathode: usize },
    BjtNpn { term_collector: usize, term_base: usize, term_emitter: f64 },
    BjtPnp { term_emitter: usize, term_base: usize, term_collector: f64 },
    NMos { term_drain: usize, term_gate: usize, term_source: f64 },
    PMos { term_source: usize, term_gate: usize, term_drain: f64 },
}

impl Component {
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
    
    fn new_current_source(tokens: Vec<&str>) -> Self {
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
	}
    }

    fn new_resistor(tokens: Vec<&str>) -> Self {
	if tokens.len() != 3 {
	    panic!("Expected three tokens for Resistor")
	}
	let term_1 = tokens[0].parse().expect("Failed to parse positive terminal");
	let term_2 = tokens[1].parse().expect("Failed to parse negative terminal");
	let resistance = tokens[2].parse().expect("Failed to parse resistance value");
	Self::Resistor {
	    term_1,
	    term_2,
	    resistance,
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

    
    
    fn new(name: &str, tokens: Vec<&str>) -> Self {
	match name {
	    "v" => Self::new_voltage_source(tokens),
	    "i" => Self::new_current_source(tokens),
	    "r" => Self::new_resistor(tokens),
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

fn parse_name_id(token: &str) -> (String, usize) {
    let name_id = token.to_ascii_lowercase();
    if name_id.len() < 2 {
	panic!("Parse error: invalid name_id {}", name_id);
    }
    let re = Regex::new(r"[a-z]+").unwrap();
    let mat = re.find(&name_id)
	.expect("Parse error: expected characters at start of {name_id}");
    let name: String = name_id
	.chars()
	.take(mat.end())
	.collect();
    let id: String = name_id
	.chars()
	.skip(mat.end())
	.collect();
    let id: usize = id.parse()
	.unwrap_or_else(|error| {
	    panic!("Failed to parse ID {id} as unsigned integer ({error})")
	});
    println!("Found name {name} and id {id}");
    
    (name, id)
}

fn parse_netlist_file(file_path: String) {

    let input = File::open(&file_path).unwrap_or_else(|error| {
	panic!("Could not open file {file_path} ({})", error.kind());
    });

    let buffered = BufReader::new(input);
    
    for line in buffered.lines().map(|ln| ln.unwrap()) {

	if line.len() == 0 {
 	    continue
	}
	
	match line.chars().nth(0).unwrap() {
	    '#' => continue,
	    _ => {},
	}

	let mut tokens = line.split_whitespace();

	// Get the component name and ID
	let name_id = tokens.next().unwrap();
	let (name, id) = parse_name_id(name_id);

	let component = Component::new(&name, tokens.collect());
	println!("{:?}", component);
	
    }
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument (the netlist file path)");
        std::process::exit(1);
    }
    let file_path = args[1].to_string();

    parse_netlist_file(file_path);
}
