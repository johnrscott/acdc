use std::{fs::File, io::BufRead, io::BufReader};

use regex::Regex;

use crate::{component::Component, mna::Mna, sparse::solve};

mod component;
mod mna;
mod sparse;

fn parse_name_id(token: &str) -> (String, usize) {
    let name_id = token.to_ascii_lowercase();
    if name_id.len() < 2 {
        panic!("Parse error: invalid name_id {}", name_id);
    }
    let re = Regex::new(r"[a-z]+").unwrap();
    let mat = re
        .find(&name_id)
        .expect("Parse error: expected characters at start of {name_id}");
    let name: String = name_id.chars().take(mat.end()).collect();
    let id: String = name_id.chars().skip(mat.end()).collect();
    let id: usize = id
        .parse()
        .unwrap_or_else(|error| panic!("Failed to parse ID {id} as unsigned integer ({error})"));
    (name, id)
}

#[derive(Debug)]
struct Instance {
    name: String,
    component: Component,
    current: Option<f64>,
}

/// Map from node indices to netlist
/// node names
#[derive(Debug)]
pub struct NodeMap {
    index_to_name: Vec<String>,
}

impl NodeMap {
    /// Make an empty node map
    fn new() -> Self {
        Self {
            index_to_name: vec![String::from("")],
        }
    }

    fn add_ground_node(&mut self, ground_name: &str) {
        if self.index_to_name[0] == "" {
            // If no ground node has been encountered yet,
            // store it here
            self.index_to_name[0] = String::from(ground_name);
        } else {
            // Else, check the ground name agrees with the
            // previously used name
            if self.index_to_name[0] != ground_name {
                panic!(
                    "Ground node name mismatch: expected {}, found {}",
                    self.index_to_name[0], ground_name
                );
            }
        }
    }

    /// Assign a terminal string to a new index, or return the index
    /// if it was already assigned.
    fn allocate_index(&mut self, node_name: &str) -> usize {
        let re = Regex::new(r"(gnd|GND|0)").unwrap();
        if re.is_match(node_name) {
            self.add_ground_node(node_name);
            0
        } else if let Some(result) = self.index_to_name.iter().position(|s| s == node_name) {
            result
        } else {
            self.index_to_name.push(String::from(node_name));
            self.index_to_name.len() - 1
        }
    }

    fn get_node_name(&self, index: usize) -> &String {
        &self.index_to_name[index]
    }
}

/// Returns (instances, mna system)
fn parse_netlist_file(file_path: String) -> (Vec<Instance>, Mna, NodeMap) {
    let input = File::open(&file_path).unwrap_or_else(|error| {
        panic!("Could not open file {file_path} ({})", error.kind());
    });

    let buffered = BufReader::new(input);

    let mut mna = Mna::new();

    // For group 2 elements, index of their current
    let mut next_free_edge = 0;

    let mut instances = Vec::new();
    let mut node_map = NodeMap::new();

    for line in buffered.lines().map(|ln| ln.unwrap()) {
        if line.len() == 0 {
            continue;
        }

        match line.chars().nth(0).unwrap() {
            '#' => continue,
            _ => {}
        }

        let mut tokens = line.split_whitespace();

        // Get the component name and ID
        let name_id = tokens.next().unwrap();
        let (name, id) = parse_name_id(name_id);

        // Collect the other argument
        let mut other_tokens: Vec<&str> = tokens.collect();
        let component = Component::new(&name, other_tokens, &mut next_free_edge, &mut node_map);

        mna.add_element_stamp(&component);

        instances.push(Instance {
            name: name_id.to_string(),
            component,
            current: None,
        });
    }

    (instances, mna, node_map)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument (the netlist file path)");
        std::process::exit(1);
    }
    let file_path = args[1].to_string();

    let (mut instances, mut mna, mut node_map) = parse_netlist_file(file_path);

    println!("{}", mna);
    println!("{:?}", node_map);
    println!("{:?}", instances);

    let num_nodes = mna.num_voltage_nodes();

    let (matrix, rhs) = mna.get_system();

    println!("{}", matrix);

    matrix.print_structure();

    println!("RHS: {:?}", rhs);

    let x = solve(matrix, rhs);
    println!();

    let voltages = &x[0..num_nodes];
    let currents = &x[num_nodes..];

    for n in 0..instances.len() {
        match instances[n].component.current_index() {
            Some(index) => instances[n].current = Some(currents[index]),
            None => {}
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

    println! {"Node voltages:"};
    println!("(0: 0 V)");
    for n in 0..voltages.len() {
        println!("{}: {} V", n + 1, voltages[n]);
    }
}
