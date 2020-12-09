use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
}

fn part_one() -> isize {
    let file: File = File::open("input.txt").unwrap();
    let mut numbers: Vec<isize> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        
    }
    //println!("{:?}", boot);
    0
}