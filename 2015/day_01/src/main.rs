use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input.txt"));
}

fn part_one(path: &str) -> isize {
    let file: File = File::open(path).unwrap();
    let mut floor = 0;
    for line in BufReader::new(file).lines() {
        line.unwrap().chars().for_each(|ch| match ch {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        });
    }
    floor
}

fn part_two(path: &str) -> usize {
    let file: File = File::open(path).unwrap();
    let mut floor = 0;
    let mut pos = 1;
    let mut result = 0;
    for line in BufReader::new(file).lines() {
        line.unwrap().chars().for_each(|ch| {
            match ch {
                '(' => {
                    floor += 1;
                }
                ')' => {
                    floor -= 1;
                }
                _ => {}
            }
            if floor == -1 && result == 0 {
                result = pos;
            }
            pos += 1;
        });
    }
    result
}
