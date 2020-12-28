use day_14::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = extract_from_file("input.txt");
    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn part_one(operations: &Vec<Cmd>) -> usize {
    let mut app: App = App::new(&operations);
    app.process();
    app.get_memory_sum()
}

fn part_two(operations: &Vec<Cmd>) -> usize {
    let mut app: App = App::new(&operations);
    app.switch_version();
    app.process();
    app.get_memory_sum()
}

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn extract_from_file(file: &str) -> Vec<Cmd> {
    lazy_static! {
        static ref MASK: Regex = Regex::new("mask = (?P<mask>[X01]{36})").unwrap();
        static ref WRITE: Regex =
            Regex::new("mem\\[(?P<address>[1-9][0-9]*)\\] = (?P<value>[0-9]+)").unwrap();
    }
    let file: File = File::open(file).unwrap();
    let mut error_counter = 0;
    let mut ops: Vec<Cmd> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let is_mask = MASK.is_match(string.as_str());
        if is_mask {
            let code = MASK.captures(string.as_str()).unwrap();
            ops.push(Cmd::SetMask(String::from(
                code.name("mask").unwrap().as_str(),
            )));
        } else {
            let is_write = WRITE.is_match(string.as_str());
            if is_write {
                let code = WRITE.captures(string.as_str()).unwrap();
                let option_address = code.name("address").unwrap().as_str().parse::<usize>();
                let option_value = code.name("value").unwrap().as_str().parse::<usize>();
                if option_address.is_err() || option_value.is_err() {
                    error_counter += 1;
                    continue;
                } else {
                    ops.push(Cmd::Write(option_address.unwrap(), option_value.unwrap()));
                }
            } else {
                error_counter += 1;
            }
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
    ops
}
