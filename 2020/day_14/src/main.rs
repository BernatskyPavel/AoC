use day_14::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let data = extract_from_file("input.txt");
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> isize {
    0
}

fn part_two() -> isize {
    0
}

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn extract_from_file(file: &str) -> (usize, Vec<(usize, usize)>) {
    lazy_static! {
        static ref MASK: Regex = Regex::new("mask = (?P<mask>[X01]{36})").unwrap();
        static ref WRITE: Regex =
            Regex::new("mem\\[(?P<address>[1-9][0-9]*)\\] = (?P<value>[0-9]+)").unwrap();
    }
    let file: File = File::open(file).unwrap();
    let mut error_counter = 0;
    let mut timestamp: usize = 0;
    let mut buses: Vec<(usize, usize)> = Vec::new();
    for line in BufReader::new(file).lines().enumerate() {
        let string = line.1.unwrap();
        match line.0 {
            0 => {
                let option_timestamp = string.parse::<usize>();
                if option_timestamp.is_err() {
                    error_counter += 1;
                    continue;
                } else {
                    timestamp = option_timestamp.unwrap();
                }
            }
            1 => {
                buses = string
                    .split(",")
                    .enumerate()
                    .filter(|bus| bus.1.parse::<usize>().is_ok())
                    .map(|bus| (bus.0, bus.1.parse::<usize>().unwrap()))
                    .collect();
            }
            _ => unimplemented!(),
        }
    }
    if error_counter != 0 {
        println!(
            "Errors: {}! Correct result is not guaranteed!",
            error_counter
        );
    } else {
        println!("File parsed with no errors!");
    };
    (timestamp, buses)
}
