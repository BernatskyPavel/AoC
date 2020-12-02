use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file: File = File::open( "input.txt").unwrap();
    let mut valid_counter = 0;
    for line in BufReader::new(file).lines(){
        let string = line.unwrap();
        let mut parts = string.split(':').collect::<Vec<&str>>();
        let rule = parts.iter().nth(0).unwrap().trim();
        let pass = parts.iter().nth(1).unwrap().trim();
        println!("{}-{}", rule, pass);
    }
    
}