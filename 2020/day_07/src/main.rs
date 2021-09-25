use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    println!("{}", part_two());
}

fn part_one() -> usize {
    lazy_static! {
        static ref BAG: Regex = Regex::new("(?P<bag>[[:word:] ?]+) bags contain").unwrap();
        static ref CONTAINS: Regex =
            Regex::new("(?P<count>[[:digit:]]+)(?P<name>[[:alpha:] ?]+) bags?").unwrap();
        static ref EMPTY: Regex = Regex::new("no other bags").unwrap();
    }
    // println!("{:?}", CONTAINS.captures_iter("light red bags contain 1 bright white bag, 2 muted yellow bags.").collect::<Vec<_>>());
    let file: File = File::open("input.txt").unwrap();
    let mut bags: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let key = String::from(
            BAG.captures(string.as_str())
                .unwrap()
                .name("bag")
                .unwrap()
                .as_str()
                .trim(),
        );
        if EMPTY.is_match(string.as_str()) {
            bags.insert(key, None);
        } else {
            let mut temp: BTreeSet<String> = BTreeSet::new();
            CONTAINS.captures_iter(string.as_str()).for_each(|cap| {
                temp.insert(String::from(cap.name("name").unwrap().as_str().trim()));
            });
            bags.insert(key, Some(temp));
        }
    }
    let mut containing_bags: BTreeSet<String> = BTreeSet::new();
    let mut buf: BTreeSet<String> = BTreeSet::new();
    buf.insert(String::from("shiny gold"));
    loop {
        buf = bags
            .iter()
            .filter(|&bag| {
                if bag.1.is_none() {
                    false
                } else {
                    !bag.1.as_ref().unwrap().is_disjoint(&buf)
                }
            })
            .map(|bag| bag.0.clone())
            .collect();
        if buf.is_empty() {
            break;
        }
        containing_bags = containing_bags.union(&buf).cloned().collect();
    }
    containing_bags.len()
}

fn part_two() -> usize {
    lazy_static! {
        static ref BAG: Regex = Regex::new("(?P<bag>[[:word:] ?]+) bags contain").unwrap();
        static ref CONTAINS: Regex =
            Regex::new("(?P<count>[[:digit:]]+)(?P<name>[[:alpha:] ?]+) bags?").unwrap();
        static ref EMPTY: Regex = Regex::new("no other bags").unwrap();
    }
    // println!("{:?}", CONTAINS.captures_iter("light red bags contain 1 bright white bag, 2 muted yellow bags.").collect::<Vec<_>>());
    let file: File = File::open("input.txt").unwrap();
    let mut bags: BTreeMap<String, Option<BTreeSet<(String, usize)>>> = BTreeMap::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let key = String::from(
            BAG.captures(string.as_str())
                .unwrap()
                .name("bag")
                .unwrap()
                .as_str()
                .trim(),
        );
        if EMPTY.is_match(string.as_str()) {
            bags.insert(key, None);
        } else {
            let mut temp: BTreeSet<(String, usize)> = BTreeSet::new();
            CONTAINS.captures_iter(string.as_str()).for_each(|cap| {
                temp.insert((
                    String::from(cap.name("name").unwrap().as_str().trim()),
                    cap.name("count")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap(),
                ));
            });
            bags.insert(key, Some(temp));
        }
    }
    let mut containing_bags: BTreeSet<(String, usize)> = BTreeSet::new();
    let mut buf: BTreeSet<(String, usize)> = BTreeSet::new();
    let mut counter = 0;
    containing_bags.insert((String::from("shiny gold"), 1));
    loop {
        containing_bags.iter().for_each(|bag|{
            let entry = bags.entry(bag.0.clone()).or_default();
            if entry.is_some() {
                entry.as_ref().unwrap().iter().for_each(|child|{
                    buf.insert((child.0.clone(), child.1 * bag.1));
                    counter += child.1 * bag.1;
                });
            }
        });
        if buf.is_empty() {
            break;
        }
        containing_bags.clone_from(&buf);
        buf.clear();
    }
    counter
}
