use std::io::{BufRead, BufReader};
use std::fs::File;

const TREE: char = '#';

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}

fn part_one() {
    let file: File = File::open("input.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut iteration = 0;
    let mut width = 0;
    let mut start = (0,0);
    for line in BufReader::new(file).lines(){
        let string = line.unwrap();
        if width == 0 {
            width = string.len();
        }
        map.push(string.chars().collect());
        iteration += 1;
    }
    let mut tree_counter = 0;
    loop {
        start.0 += 3;
        start.1 += 1;
        if start.0 >= width {
            start.0 -= width;
        }
        if map[start.1][start.0] == TREE {
            tree_counter += 1;
        }
        if start.1 == iteration - 1 {
            break;
        }
    }

    println!("{}", tree_counter);
}

fn part_two() {
    let file: File = File::open("input.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut iteration = 0;
    let mut width = 0;  
    for line in BufReader::new(file).lines(){
        let string = line.unwrap();
        if width == 0 {
            width = string.len();
        }
        map.push(string.chars().collect());
        iteration += 1;
    }
    let mut result = 1;
    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut i = 0;
    loop {
        let mut start = (0,0);
        let mut tree_counter: usize = 0;
        loop {
            start.0 += slopes[i].0;
            start.1 += slopes[i].1;
            if start.0 >= width {
                start.0 -= width;
            }
            if map[start.1][start.0] == TREE {
                tree_counter += 1;
            }
            if start.1 == iteration - 1 {
                break;
            }
        }
        i+=1;
        result *= tree_counter;
        if i == slopes.len() {
            break;
        }
    }

    println!("{}", result);
}