fn main() {
    println!("{:?}", part_two("map.txt", 200));
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ROUND: f64 = 10000.0;

#[allow(dead_code)]
fn part_one(file_path: &str) -> ((isize, isize), usize) {
    let asteroids_map: Vec<(isize, isize)> = extract_map_from_file(file_path);
    search_for_best_place(&asteroids_map)
}

fn extract_map_from_file(file_path: &str) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = Vec::new();
    let mut row = 0;
    let mut column = 0;
    let file: File = File::open(file_path).unwrap();
    for line in BufReader::new(file).lines() {
        for ch in line.unwrap().chars() {
            if ch == '#' {
                result.push((column, row));
            }
            column += 1;
        }
        column = 0;
        row += 1;
    }
    result
}

fn search_for_best_place(map: &Vec<(isize, isize)>) -> ((isize, isize), usize) {
    let mut max: usize = 0;
    let mut max_pos: (isize, isize) = (0, 0);
    map.iter().for_each(|&a| {
        let visible = count_visible(a, &map);
        if max < visible {
            max_pos = a;
            max = visible;
        };
    });
    (max_pos, max)
}

fn count_visible(candidate: (isize, isize), map: &Vec<(isize, isize)>) -> usize {
    let mut tans_of_lower = Vec::new();
    let mut tans_of_upper = Vec::new();
    for &b in map.iter() {
        if candidate != b {
            let tan = (candidate.1 - b.1) as f64 / (candidate.0 - b.0) as f64;
            if candidate.1 >= b.1 {
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
    tans_of_lower.len() + tans_of_upper.len()
}

fn map_of_visible(candidate: (isize, isize), map: &Vec<(isize, isize)>) -> (Vec<(isize, (isize, isize))>, Vec<(isize, (isize, isize))>, usize) {
    //let mut result = Vec::new();
    let mut left_part: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    let mut right_part: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for &b in map.iter() {
        if candidate != b {
            let tan = (((candidate.1 - b.1) as f64 / (candidate.0 - b.0) as f64) * ROUND).round()
            as isize;
            if candidate.0 > b.0 {
                left_part.entry((tan.signum(), tan)).or_insert(Vec::new()).push(b);
            } else {
                right_part.entry((tan.signum(), tan)).or_insert(Vec::new()).push(b);
            }
        }
    }

    let mut left_map = left_part.iter_mut().map(|x|{
        //let mut values = x.clone();
        x.1.sort_by(|&a, &b|{
            (b.0 - candidate.0 + b.1 - candidate.1).partial_cmp(&(a.0 - candidate.0 + a.1 - candidate.1)).unwrap()
        });
        (x.0.1, x.1.get(0).unwrap().clone())
    }).collect::<Vec<(isize, (isize, isize))>>();
    let mut right_map = right_part.iter_mut().map(|x|{
        x.1.sort_by(|&a, &b|{
            (b.0 - candidate.0 + b.1 - candidate.1).partial_cmp(&(a.0 - candidate.0 + a.1 - candidate.1)).unwrap()
        });
        (x.0.1, x.1.get(0).unwrap().clone())
    }).collect::<Vec<(isize, (isize, isize))>>();
    right_map.sort_by_key(|x|{
        x.0
    });
    left_map.sort_by_key(|x|{
        x.0
    });
    let len = left_map.len() + right_map.len();
    (left_map, right_map, len)
}

fn part_two(file_path: &str, index: usize) -> Option<(isize, isize)> {
    let mut asteroids_map: Vec<(isize, isize)> = extract_map_from_file(file_path);
    let station = search_for_best_place(&asteroids_map);
    let mut maps = map_of_visible(station.0, &asteroids_map);
    let mut deleted = 0;
    loop {
        for asteroid in maps.1.iter() {
            if deleted == index - 1 {
                return Some(asteroid.1);
            }
            asteroids_map = asteroids_map.iter().filter(|&&p| {
                p != asteroid.1
            }).copied().collect();
            deleted += 1;
        }
        for asteroid in maps.0.iter() {
            if deleted == index - 1 {
                return Some(asteroid.1);
            }
            asteroids_map = asteroids_map.iter().filter(|&&p| {
                p != asteroid.1
            }).copied().collect();
            deleted += 1;
        }
        maps = map_of_visible(station.0, &asteroids_map);
        if maps.2 == 0 {
            break;
        }
    };
    None
}
