use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
}

fn part_one() -> u16 {
    lazy_static! {
        static ref BAG: Regex = Regex::new("(?P<bag>[[:word:] ?]+) bags contain").unwrap();
        static ref CONTAINS: Regex = Regex::new("(?P<count>[[:digit:]]+)(?P<name>[[:alpha:] ?]+) bags?").unwrap();
        static ref EMPTY: Regex = Regex::new("no other bags").unwrap();
    }
    //println!("{:?}", RE.captures_iter("light red bags contain 1 bright white bag, 2 muted yellow bags.").collect::<Vec<_>>());
    let file: File = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
    }
    0
}
