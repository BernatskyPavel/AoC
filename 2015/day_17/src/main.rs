use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part one: {}", part_one("input.txt", 150));
    println!("Part two: {}", part_two("input.txt", 150));
}

fn part_one(path: &str, limit: usize) -> usize {
    let file: File = File::open(path).unwrap();
    let mut bottles = vec![];
    for line in BufReader::new(file).lines().flatten() {
        if let Ok(num) = line.parse::<usize>() {
            bottles.push(num);
        }
    }

    let mut counter = 0;
    for i in 1..1 << bottles.len() {
        let mut temp = i;
        let mut sum = 0;
        for bottle in &bottles {
            if temp % 2 == 1 {
                sum += *bottle;
            }
            temp >>= 1;
        }
        if sum == limit {
            counter += 1;
        }
    }
    counter
}

fn part_two(path: &str, limit: usize) -> usize {
    let file: File = File::open(path).unwrap();
    let mut bottles = vec![];
    for line in BufReader::new(file).lines().flatten() {
        if let Ok(num) = line.parse::<usize>() {
            bottles.push(num);
        }
    }

    let mut counter = 0;
    let mut min = bottles.len();
    for i in 1_usize..1 << bottles.len() {
        let mut temp = i;
        let mut sum = 0;
        for bottle in &bottles {
            if temp % 2 == 1 {
                sum += *bottle;
            }
            temp >>= 1;
        }

        match (sum, i.count_ones() as usize) {
            (sum, len) if len < min && sum == limit => {
                min = len;
                counter = 1;
            }
            (sum, len) if len == min && sum == limit => {
                counter += 1;
            }
            (_, _) => {}
        }
    }
    counter
}

#[test]
fn test_part_one() {
    assert_eq!(part_one("test.txt", 25), 4);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two("test.txt", 25), 3);
}
