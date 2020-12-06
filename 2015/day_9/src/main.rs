use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    part_one();
}

fn part_one() {
    let file: File = File::open("input.txt").unwrap();
    lazy_static! {
        static ref RE: Regex =
            Regex::new("(?P<from>[A-Za-z]+) to (?P<to>[A-Za-z]+) = (?P<dist>[1-9][0-9]*)").unwrap();
    }
    let mut cities: BTreeSet<String> = BTreeSet::new();
    let mut cities_dist: BTreeMap<(String, String), usize> = BTreeMap::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let captures = RE.captures(string.as_str()).unwrap();
        let (from, to, dist) = (
            captures.name("from").unwrap().as_str(),
            captures.name("to").unwrap().as_str(),
            captures.name("dist").unwrap().as_str(),
        );
        cities.insert(String::from(from));
        cities.insert(String::from(to));
        cities_dist.insert(
            (String::from(from), String::from(to)),
            dist.parse::<usize>().unwrap(),
        );
    }
    let mut min_distance = usize::MAX;
    let mut max_distance = 0;
    cities.iter().permutations(cities.len()).for_each(|path| {
        let mut path_lenght = 0;
        path.windows(2).for_each(|pair| {
            if cities_dist.contains_key(&(pair[0].to_string(), pair[1].to_string())) {
                path_lenght += cities_dist
                    .get(&(pair[0].to_string(), pair[1].to_string()))
                    .unwrap();
            } else {
                path_lenght += cities_dist
                    .get(&(pair[1].to_string(), pair[0].to_string()))
                    .unwrap();
            }
        });
        if min_distance > path_lenght {
            min_distance = path_lenght;
        }
        if max_distance < path_lenght {
            max_distance = path_lenght;
        }
    });
    println!("{}-{}", min_distance, max_distance);
}
