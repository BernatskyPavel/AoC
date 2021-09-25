use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let file: File = File::open( "input.txt").unwrap();
    let mut valid_counter = 0;
    for line in BufReader::new(file).lines(){
        let string = line.unwrap();
        let parts = string.split(':').collect::<Vec<&str>>();
        let rule: Vec<&str> = parts.iter().nth(0).unwrap().trim().split(|ch| {
            " -".contains(ch)
        }).collect();
        let (mut min, mut max, mut symbol) = (0,0,'0');
        rule.iter().enumerate().for_each(|pair|{
            match pair.0 {
                0 => {
                    min = pair.1.parse::<usize>().unwrap();
                },
                1 => {
                    max = pair.1.parse::<usize>().unwrap();
                },
                2 => {
                    symbol = pair.1.chars().nth(0).unwrap();  
                },
                _ => {},
            }
        });
        let pass = parts.iter().nth(1).unwrap().trim();
        let counter = pass.matches(symbol).collect::<Vec<_>>().len();
        if counter >= min && counter <= max {
            valid_counter += 1;
        }
    }
    valid_counter
}

fn part_two() -> usize {
    let file: File = File::open( "input.txt").unwrap();
    let mut valid_counter = 0;
    for line in BufReader::new(file).lines(){
        let string = line.unwrap();
        let parts = string.split(':').collect::<Vec<&str>>();
        let rule: Vec<&str> = parts.iter().nth(0).unwrap().trim().split(|ch| {
            " -".contains(ch)
        }).collect();
        let (mut first, mut second, mut symbol) = (0,0,'0');
        rule.iter().enumerate().for_each(|pair|{
            match pair.0 {
                0 => {
                    first = pair.1.parse::<usize>().unwrap();
                },
                1 => {
                    second = pair.1.parse::<usize>().unwrap();
                },
                2 => {
                    symbol = pair.1.chars().nth(0).unwrap();  
                },
                _ => {},
            }
        });
        let pass = parts.iter().nth(1).unwrap().trim();
        let counter = pass.match_indices(symbol).filter(|pair| {
            pair.0 == first - 1 || pair.0 == second - 1
        }).collect::<Vec<_>>().len();
        if counter == 1 {
            valid_counter += 1;
        }
    }
    valid_counter
}