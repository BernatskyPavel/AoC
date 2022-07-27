use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input.txt"));
}

fn part_one(path: &str) -> usize {
    let file: File = File::open(path).unwrap();
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
    code_length - memory_length
}

fn part_two(path: &str) -> usize {
    let file: File = File::open(path).unwrap();
    let mut code_length = 0;
    let mut memory_length = 0;
    for line in BufReader::new(file).lines() {
        let mut string = line.unwrap();
        code_length += string.len();
        string = string.replace('\\', "\\\\");
        string = string.replace('\"', "\\\"");
        memory_length += string.len() + 2;
    }
    memory_length - code_length
}
