use std::{fs::File, io::BufReader, io::BufRead};

use regex::Regex;

use crate::{component::Component, mna::Mna, sparse::solve};

mod component;
mod sparse;
mod mna;

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
    (name, id)
}


#[derive(Debug)]
struct Instance {
    name: String,
    component: Component,
    current: Option<f64>,
}


/// Returns (instances, mna system)
fn parse_netlist_file(
    file_path: String
) -> (Vec<Instance>, Mna) {
    
    let input = File::open(&file_path).unwrap_or_else(|error| {
	panic!("Could not open file {file_path} ({})", error.kind());
    });

    let buffered = BufReader::new(input);

    let mut mna = Mna::new();

    // For group 2 elements, index of their current
    let mut next_free_edge = 0;

    let mut instances = Vec::new();
    
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
	let component = Component::new(&name, other_tokens, &mut next_free_edge);

	mna.add_element_stamp(&component);

	instances.push(Instance {
	    name: name_id.to_string(),
	    component,
	    current: None,
	});
    }

    (instances, mna)
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument (the netlist file path)");
        std::process::exit(1);
    }
    let file_path = args[1].to_string();

    let (mut instances, mut mna) = parse_netlist_file(file_path);

    println!("{}", mna);
    
    let num_nodes = mna.num_nodes();
	
    let (matrix, rhs) = mna.get_system();

    println!("{}", matrix);
    
    let x = solve(matrix, rhs);
    println!();
    
    let voltages = &x[0..num_nodes];
    let currents = &x[num_nodes..];

    for n in 0..instances.len() {
	match instances[n].component.current_index() {
	    Some(index) => {
		instances[n].current = Some(currents[index])
	    },
	    None => {},
	}
    }

    

    // Print the components
    println!("Components:");
    for inst in instances.iter() {
	print!("{}: {}", inst.name, inst.component);
	match inst.current {
	    Some(current) => println!(" ({} A)", current),
	    None => println!(),
	}
    }
    println!();

    println!{"Node voltages:"};
    println!("(0: 0 V)");
    for n in 0..voltages.len() {
	println!("{}: {} V", n+1, voltages[n]);
    }
    
}
