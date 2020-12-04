use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
}

fn part_one() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut valid_counter = 0;
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let all_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut pass: BTreeMap<&str, &str> = BTreeMap::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        if string.is_empty() {
            let keys = pass.keys().collect::<Vec<&&str>>();
            let is_all_presented = all_fields.iter().all(|key| {
                keys.contains(&key)
            });
            if is_all_presented {
                valid_counter += 1;
            } else {
                if required_fields.iter().all(|key|{
                    keys.contains(&key)
                }) {
                    valid_counter +=1;
                }
            }
            pass.clear();
        } else {
            string.split_whitespace().for_each(|fields| {
                let values = fields.split(":").collect::<Vec<&str>>();
                pass.insert(values[0].clone(), values[1]);
            });
        }
    }
    valid_counter
}