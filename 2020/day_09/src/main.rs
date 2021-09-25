use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    println!("{}", part_one(25));
    println!("{}", part_two(25));
}

fn part_one(preamble: usize) -> isize {
    let mut numbers: Vec<isize> = Vec::new();
    {
        let file: File = File::open("input.txt").unwrap();

        for line in BufReader::new(file).lines() {
            let string = line.unwrap();
            numbers.push(string.parse::<isize>().unwrap());
        }
    }
    let mut i = 0;
    loop {
        let mut is_valid = false;
        let temp = numbers[preamble + i];
        numbers[i..preamble + i].iter().enumerate().for_each(|num| {
            let temp = numbers[i..preamble + i]
                .iter()
                .enumerate()
                .any(|other| temp == (num.1 + other.1) && num.0 != other.0);
            if temp {
                is_valid = true;
            }
        });
        i += 1;

        if !is_valid {
            break temp;
        }
    }
    //println!("{:?}", boot);
}

fn part_two(preamble: usize) -> isize {
    let mut numbers: Vec<isize> = Vec::new();
    {
        let file: File = File::open("input.txt").unwrap();

        for line in BufReader::new(file).lines() {
            let string = line.unwrap();
            numbers.push(string.parse::<isize>().unwrap());
        }
    }
    let mut i = 0;
    let invalid_number = loop {
        let mut is_valid = false;
        let temp = numbers[preamble + i];
        numbers[i..preamble + i].iter().enumerate().for_each(|num| {
            let temp = numbers[i..preamble + i]
                .iter()
                .enumerate()
                .any(|other| temp == (num.1 + other.1) && num.0 != other.0);
            if temp {
                is_valid = true;
            }
        });
        i += 1;

        if !is_valid {
            break temp;
        }
    };
    i = 0;
    loop {
        let mut min = isize::MAX;
        let mut max = isize::MIN;
        let mut sum = 0;
        let mut j = 0;
        let is_founded = loop {
            if min > numbers[i + j] {
                min = numbers[i + j];
            }
            if max < numbers[i + j] {
                max = numbers[i + j];
            }
            sum += numbers[i + j];
            if sum > invalid_number {
                break false;
            }
            if sum == invalid_number {
                break true;
            }
            j += 1;
        };
        i += 1;

        if is_founded {
            break min + max;
        }
    }
}
