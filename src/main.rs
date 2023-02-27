use std::{fs::File, io::BufReader, io::BufRead};

use regex::Regex;

fn parse_name_id(token: String) -> (String, usize) {
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

	let mut tokens = line.split_ascii_whitespace();
	// Get the component name and ID
	let name_id = tokens.next().unwrap();
	let (name, id) = parse_name_id(name_id);
	match name.as_str() {
	    "v" => println!("Voltage"),
	    "i" => println!("Current"),
	    "r" => println!("Resistor"),
	    "c" => println!("Capacitor"),
	    "l" => println!("Inductor"),
	    "d" => println!("Diode"),
	    "qn" => println!("NPN"),
	    "qp" => println!("PNP"),
	    "mn" => println!("NMOS"),
	    "mp" => println!("PMOS"),
	    &_ => panic!("Found unexpected name {name}"),
	}
	
	for token in tokens {
	    println!("T: {token}");
	}	
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
