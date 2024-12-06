use std::fs;
use std::env;

use crate::help;

pub fn read_file(_path: &str) -> Option<Vec<String>> {
    println!("Reading file from path {_path}");

    let content = match fs::read_to_string(_path) {
        Ok(fc) => fc, 
        Err(_error) => panic!("Could not read file!"),
    };
    let splitted_content: Vec<String> = content.split("\n").map(String::from).collect();
    
    Some(splitted_content.clone())
}

pub fn extract_input_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No input has been provided!");
        help::print_usage();
        
        panic!() 
    }

    args[1].clone()
}
