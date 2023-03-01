use std::{fs::File, io::BufReader, io::BufRead};

use regex::Regex;

use crate::component::Component;

mod component;

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

	// Collect the other argument
	let mut other_tokens: Vec<&str> = tokens.collect();
	
	// Check if the last element is a group 2 specifier
	let group2: bool;
	if other_tokens.last().unwrap().to_string() == "G2" {
	    group2 = true;
	    other_tokens.pop();
	} else {
	    group2 = false;
	}
	
	let component = Component::new(&name, other_tokens, group2);
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
