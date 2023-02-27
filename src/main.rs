use std::{fs::File, io::BufReader, io::{BufRead, Lines}, str::FromStr};

use regex::Regex;

#[derive(Debug)]
enum Component {
    VoltageSource { term_pos: usize, term_neg: usize, voltage: f64 },
    CurrentSource,
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    BjtNpn,
    BjtPnp,
    NMos,
    PMos,
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
    
    fn new(name: &str, tokens: Vec<&str>) -> Self {
	match name {
	    "v" => Self::new_voltage_source(tokens),
	    "i" => Self::CurrentSource,
	    "r" => Self::Resistor,
	    "c" => Self::Capacitor,
	    "l" => Self::Inductor,
	    "d" => Self::Diode,
	    "qn" => Self::BjtNpn,
	    "qp" => Self::BjtPnp,
	    "mn" => Self::NMos,
	    "mp" => Self::PMos,
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
        println!("Please provide exactly one argument (the matrix file path)");
        std::process::exit(1);
    }
    let file_path = args[1].to_string();

    parse_netlist_file(file_path);
}
