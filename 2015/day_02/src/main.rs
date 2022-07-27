use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input.txt"));
}

fn part_one(path: &str) -> usize {
    let file: File = File::open(path).unwrap();
    let mut result = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let dimensions = string
            .split('x')
            .map(|size| size.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let sides = vec![
            dimensions[0] * dimensions[1],
            dimensions[1] * dimensions[2],
            dimensions[2] * dimensions[0],
        ];
        let min = sides.iter().min().unwrap();
        result += sides.iter().sum::<usize>() * 2;
        result += min;
    }
    result
}

fn part_two(path: &str) -> usize {
    let file: File = File::open(path).unwrap();
    let mut result = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let mut dimensions = string
            .split('x')
            .map(|size| size.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        dimensions.sort_unstable();
        result += 2 * (dimensions[0] + dimensions[1]);
        result += dimensions.iter().product::<usize>();
    }
    result
}
