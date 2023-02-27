use std::{fs::File, io::BufReader, io::BufRead};

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument (the matrix file path)");
        std::process::exit(1);
    }
    let file_path = args[1].to_string();

    let input = File::open(file_path)
	.expect("Failed to open file");

    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {
	println!("{}", line.unwrap());
    }

    
}
