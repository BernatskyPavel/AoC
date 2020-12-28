use std::fs::File;
use std::io::{BufRead, BufReader};
use day_16::{Plane, Ticket, Field};

fn main() {
    let mut data = extract_from_file("input.txt");
    //println!("{}", part_one(&data));
    println!("{}", part_two(&mut data));
}

fn part_one(plane: &Plane) -> usize {
    plane.invalid_rate()
}

fn part_two(plane: &mut Plane) -> usize {
    plane.define_fields();
    plane.calc_fields()
}

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn extract_from_file(file: &str) -> Plane {
    lazy_static! {
        static ref FIELD: Regex = Regex::new("(?P<field>[a-zA-Z ]*): (?P<lowest_one>[0-9]+)-(?P<highest_one>[0-9]+) or (?P<lowest_two>[0-9]+)-(?P<highest_two>[0-9]+)").unwrap();
        static ref SKIP: Regex = Regex::new("your ticket:|nearby tickets:").unwrap();
        static ref TICKET: Regex = Regex::new("^([0-9,]+)").unwrap();
    }
    let file: File = File::open(file).unwrap();
    let mut error_counter = 0;
    let mut fields: Vec<Field> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        if string.is_empty() {
            continue;
        }
        let is_skip = SKIP.is_match(string.as_str());
        if is_skip {
            continue;
        }
        let is_field = FIELD.is_match(string.as_str());
        if is_field {
            let code = FIELD.captures(string.as_str()).unwrap();
            fields.push(Field::new(
                &String::from(code.name("field").unwrap().as_str()),
                code.name("lowest_one")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                code.name("highest_one")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                code.name("highest_two")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                code.name("lowest_two")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            ));
        } else {
            let is_ticket = TICKET.is_match(string.as_str());
            if is_ticket {
                tickets.push(Ticket::new(
                    &string
                        .clone()
                        .split(",")
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                ));
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
    Plane::new(&fields, &tickets)
}
