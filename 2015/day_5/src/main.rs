use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::collections::BTreeSet;

fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut result = 0;
    let bad_strings = ["ab", "cd", "pq", "xy"];
    for line in BufReader::new(file).lines() {
        let mut is_twice = false;
        let mut is_bad = false;
        let string = line.unwrap();
        let vowels: Vec<_> = string.matches(|ch| "aeiou".contains(ch)).collect();
        if vowels.len() < 3 {
            continue;
        }
        string
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .for_each(|win| {
                if win[0] == win[1] {
                    is_twice = true;
                }
            });
        if !is_twice {
            continue;
        }
        bad_strings.iter().for_each(|bad| {
            if string.contains(bad) {
                is_bad = true;
            }
        });
        if !is_bad {
            result += 1;
        }
    }
    result
}

use std::collections::BTreeSet;

fn part_two() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut result = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let chars = string.chars().collect::<Vec<char>>();
        let mut second_rule = chars
            .windows(3)
            .skip_while(|win| !(win[0] == win[2] && win[0] != win[1]));
        if second_rule.next().is_none() {
            continue;
        }
        let mut pairs: BTreeSet<String> = BTreeSet::new();
        chars.windows(2).for_each(|win| {
            let temp = format!("{}{}", win[0], win[1]);
            pairs.insert(temp.clone());
        });
        let mut first_rule = pairs.iter().skip_while(|pair|
            string.matches(*pair).collect::<Vec<_>>().len() < 2
        );
        if first_rule.next().is_some() {
            result += 1;
        };
    }
    result
}
