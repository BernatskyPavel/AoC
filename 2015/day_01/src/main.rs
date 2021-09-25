use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> isize {
    let file: File = File::open("input.txt").unwrap();
    let mut floor = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        string.chars().for_each(|ch| match ch {
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

fn part_two() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut floor = 0;
    let mut pos = 1;
    let mut result = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        string.chars().for_each(|ch| {
            match ch {
                '(' => {
                    floor += 1;
                }
                ')' => {
                    floor -= 1;
                }
                _ => {}
            }
            if floor == -1 {
                if result == 0 {
                    result = pos;
                }
            }
            pos += 1;
        });
    }
    result
}
