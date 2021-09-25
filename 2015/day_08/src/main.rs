use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}

fn part_one() {
    let file: File = File::open("input.txt").unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new("\\\\x[0-9A-Fa-f]{2}").unwrap();
    }
    println!("{}", RE.is_match(r"\x13"));
    let mut code_length = 0;
    let mut memory_length = 0;
    for line in BufReader::new(file).lines() {
        let mut string = line.unwrap();
        code_length += string.len();
        string = RE.replace_all(string.as_str(), "$").into_owned();
        string = string.replace(r"\\", "$");
        string = string.replace("\\\"", "$");
        memory_length += string.len() - 2;
    }
    println!("{}", code_length - memory_length);
}

fn part_two() {
    let file: File = File::open("input.txt").unwrap();
    let mut code_length = 0;
    let mut memory_length = 0;
    for line in BufReader::new(file).lines() {
        let mut string = line.unwrap();
        code_length += string.len();
        string = string.replace("\\", "\\\\");
        string = string.replace("\"", "\\\"");
        memory_length += string.len() + 2;
    }
    println!("{}", memory_length - code_length);
}