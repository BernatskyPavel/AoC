use itertools::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input2.txt"));
}

fn task(file: &str, number_of_persons: usize) -> isize {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<from>[A-Za-z]+) would (?P<sign>lose|gain) (?P<value>\d+) happiness units by sitting next to (?P<to>[A-Za-z]+).").unwrap();
    }
    let mut person_map = vec![vec![0; number_of_persons]; number_of_persons];
    let mut person_list: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            let from = String::from(&cap["from"]);
            let i = if let Ok(index) = person_list.binary_search(&from) {
                index
            } else {
                person_list.push(from);
                person_list.len() - 1
            };
            let to = String::from(&cap["to"]);
            let j = if let Ok(index) = person_list.binary_search(&to) {
                index
            } else {
                person_list.push(to);
                person_list.len() - 1
            };
            let sign = match &cap["sign"] {
                "lose" => -1,
                "gain" => 1,
                _ => unimplemented!(),
            };
            let value = cap["value"].parse::<isize>().unwrap();
            person_map[i][j] = sign * value;
        });
    }

    person_list
        .iter()
        .enumerate()
        .map(|person| person.0)
        .permutations(number_of_persons)
        .map(|persons| {
            persons
                .windows(2)
                .map(|window| person_map[window[0]][window[1]] + person_map[window[1]][window[0]])
                .sum::<isize>()
                + person_map[persons[0]][*persons.last().unwrap()]
                + person_map[*persons.last().unwrap()][persons[0]]
        })
        .max()
        .unwrap()
}

fn part_one(file: &str) -> isize {
    task(file, 8)
}

fn part_two(file: &str) -> isize {
    task(file, 9)
}
