use std::{fs::File, io::BufReader, io::BufRead};

fn parse_netlist_file(file_path: String) {

    let input = File::open(&file_path).unwrap_or_else(|error| {
	panic!("Could not open file {file_path} ({})", error.kind());
    });

    let buffered = BufReader::new(input);
    
    for line in buffered.lines().map(|ln| ln.unwrap()) {
	let mut tokens = line.split_ascii_whitespace();
	// Get the component name and ID
	let name_id = tokens.next()
	    .expect("Failed to read component type and ID")
	    .to_ascii_lowercase();
	if name_id.len() < 2 {
	    panic!("Parse error: invalid name_id {}", name_id);
	}
	let name = name_id.chars().nth(0).unwrap();
	let id: String = name_id
	    .chars()
	    .skip(1)
	    .collect();
	let id: usize = id.parse()
	    .unwrap_or_else(|error| {
		panic!("Failed to parse ID {id} as unsigned integer ({error})")
	    });
	println!("Found name {name} and id {id}");
	
	
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
