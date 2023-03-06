extern crate glob;

use std::fs::File;
use std::io::prelude::*;
use glob::glob;
use std::env;
use regex::Regex;
use std::fs;

fn main(){
    
    let filepaths: Vec<String> = Vec::new();

    // PARSE ARGS
    let params: Vec<String> = env::args().collect();
    let glob_arg = params[1].clone();
    let regex_arg = params[2].clone();
    
    let files: Vec<String> = glob(&glob_arg).unwrap().map(|a| a.unwrap().display().to_string()).collect();

    // let contents: Vec<String> = files.iter().map(|a| fs::read_to_string(a).unwrap()).collect();
    
    // let filtered: Vec<_> = contents.iter().filter(|a| Regex::new(&regex_arg).unwrap().is_match(a)).collect();
    
    let filt_contents: Vec<_> = files.iter()
        .filter(
            |a| Regex::new(&regex_arg).unwrap().is_match(
                &fs::read_to_string(a).unwrap()
            ))
        .collect();

    // println!("{:?}", files);
    // println!("{:?}", contents);
    // println!("{:?}", filtered);
    println!("{:?}", filt_contents);

}