use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Part one: {:?}", part_one("input.txt"));
    println!("Part two: {:?}", part_two("input.txt"));
    //part_one("input.txt");
    //part_two("input.txt");
}

fn part_one(file: &str) -> Option<String> {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<prop>\w+: \d+)").unwrap();
        static ref NUM: Regex = Regex::new(r"Sue (?P<num>\d+):").unwrap();
    }

    let gift_list: HashMap<&str, usize> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("cars", 2),
        ("trees", 3),
        ("perfumes", 1),
    ]);

    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let mut is_match = true;
        RE.captures_iter(string.as_str()).for_each(|cap| {
            let key_value_pair = cap["prop"]
                .split(':')
                .map(|elem| elem.trim())
                .collect::<Vec<_>>();
            if gift_list.contains_key(key_value_pair[0]) {
                let value = key_value_pair[1].parse::<usize>().unwrap();
                if value != gift_list[key_value_pair[0]] {
                    is_match = false;
                }
            }
        });
        if is_match {
            return Some(String::from(&NUM.captures(&string).unwrap()["num"]));
        }
    }
    None
}

fn part_two(file: &str) -> Option<String> {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<prop>\w+: \d+)").unwrap();
        static ref NUM: Regex = Regex::new(r"Sue (?P<num>\d+):").unwrap();
    }

    let gift_list: HashMap<&str, usize> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("cars", 2),
        ("trees", 3),
        ("perfumes", 1),
    ]);

    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let mut is_match = true;
        RE.captures_iter(string.as_str()).for_each(|cap| {
            let key_value_pair = cap["prop"]
                .split(':')
                .map(|elem| elem.trim())
                .collect::<Vec<_>>();
            if gift_list.contains_key(key_value_pair[0]) {
                let value = key_value_pair[1].parse::<usize>().unwrap();
                match key_value_pair[0] {
                    key if key == "cats" || key == "trees" => {
                        if value <= gift_list[key] {
                            is_match = false;
                        }
                    }
                    key if key == "pomeranians" || key == "goldfish" => {
                        if value >= gift_list[key] {
                            is_match = false;
                        }
                    }
                    key => {
                        if value != gift_list[key] {
                            is_match = false;
                        }
                    }
                };
            }
        });
        if is_match {
            return Some(String::from(&NUM.captures(&string).unwrap()["num"]));
        }
    }
    None
}
