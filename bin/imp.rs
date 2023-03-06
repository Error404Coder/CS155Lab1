extern crate glob;

use std::fs::File;
use std::io::prelude::*;
use glob::glob;
use std::env;
use regex::Regex;
use std::process;

fn main(){
    
    let mut filepaths: Vec<String> = Vec::new();

    // PARSE ARGS
    let params: Vec<String> = env::args().collect();
    let glob_arg = params[1].clone();
    let regex_arg = params[2].clone();

    // FIND FILES via GLOB
    for entry in glob(&glob_arg).expect("Invalid glob path"){
        match entry {
            Ok(path) => {
                // READ FILES 
                let mut file = File::open(path.display().to_string()).expect("Error opening file (1).");
                let mut content = String::new();

                file.read_to_string(&mut content)
                    .expect("Error in reading file.");

                // FILTER FILE via REGEX 
                let re = Regex::new(&regex_arg).expect("Invalid regex string");
                if re.is_match(&content){
                    filepaths.push(path.display().to_string());
                }
            },
            Err(_e) => {
                println!("Invalid glob path");
                process::exit(1);
            },
        }
    }

    // // TEST REGEX STRING
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // println!("{:?}", re.is_match("2014-20-99"));

    filepaths.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("{:?}", filepaths);

}