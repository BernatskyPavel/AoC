use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeSet;

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}

fn part_one() {
    let file: File = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut group: BTreeSet<char> = BTreeSet::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        if string.is_empty() {
            result += group.len();
            group.clear();
        } else {
            string.chars().for_each(|ch|{
                group.insert(ch);
            });
        }
    }
    println!("{}", result);
}

fn part_two() {
    let file: File = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut group: BTreeSet<char> = BTreeSet::new();
    let mut prev = false;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        if string.is_empty() {
            result += group.len();
            group.clear();
            prev = false;
        } else {
            if !prev {
                string.chars().for_each(|ch|{
                    group.insert(ch);
                });
                prev = true;
            } else {
                let mut temp: BTreeSet<char> = BTreeSet::new();
                string.chars().for_each(|ch|{
                    temp.insert(ch);
                });
                group = group.intersection(&temp).cloned().collect();
            }
            
        }
    }
    println!("{}", result);
}
