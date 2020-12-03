use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut houses: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut current_house = (0, 0);
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        string.chars().for_each(|ch| {
            match ch {
                '^' => {
                    current_house.1 += 1;
                }
                '>' => {
                    current_house.0 += 1;
                }
                '<' => {
                    current_house.0 -= 1;
                }
                'v' => {
                    current_house.1 -= 1;
                }
                _ => {}
            };
            houses.insert(current_house);
        });
    }
    houses.len()
}

fn part_two() -> usize {
    let file: File = File::open("input.txt").unwrap();
    let mut houses: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut santa_house = (0, 0);
    let mut robo_house = (0, 0);
    houses.insert((0,0));
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        string.chars().collect::<Vec<char>>().chunks(2).for_each(|ch| {
            match ch[0] {
                '^' => {
                    santa_house.1 += 1;
                }
                '>' => {
                    santa_house.0 += 1;
                }
                '<' => {
                    santa_house.0 -= 1;
                }
                'v' => {
                    santa_house.1 -= 1;
                }
                _ => {}
            };
            houses.insert(santa_house);
            match ch[1] {
                '^' => {
                    robo_house.1 += 1;
                }
                '>' => {
                    robo_house.0 += 1;
                }
                '<' => {
                    robo_house.0 -= 1;
                }
                'v' => {
                    robo_house.1 -= 1;
                }
                _ => {}
            };
            houses.insert(robo_house);
        });
    }
    houses.len()
}
