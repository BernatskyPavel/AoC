fn main() {
    part_one();
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() {
    let mut asteroids: Vec<(isize, isize)> = Vec::new();
    let mut row = 0;
    {
        let mut column = 0;
        let file: File = File::open("map.txt").unwrap();
        for line in BufReader::new(file).lines() {
            for ch in line.unwrap().chars() {
                if ch == '#' {
                    asteroids.push((column, row));
                }
                column += 1;
            }
            column = 0;
            row += 1;
        }
    }
    let mut number_of_visible: HashMap<(isize, isize), usize> = HashMap::new();
    asteroids.iter().for_each(|&a| {
        let mut tans_of_lower = Vec::new();
        let mut tans_of_upper = Vec::new();
        for &b in asteroids.iter() {
            if a != b {
                let tan = (a.1 - b.1) as f64 / (a.0 - b.0) as f64;
                if a.1 >= b.1 {
                    if !tans_of_upper.contains(&(tan.signum(), tan)) {
                        tans_of_upper.push((tan.signum(), tan));
                    }
                } else {
                    if !tans_of_lower.contains(&(tan.signum(), tan)) {
                        tans_of_lower.push((tan.signum(), tan));
                    }
                }
            }
        }
        number_of_visible.insert(a, tans_of_lower.len() + tans_of_upper.len());
    });
    println!("{:?}", number_of_visible.values().max());
}
