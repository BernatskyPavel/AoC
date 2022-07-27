use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input.txt"));
}

fn part_one(path: &str) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    let file: File = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let parts = string.split_whitespace().collect::<Vec<&str>>();
        let new_state;
        match parts[0] {
            "turn" => {
                match parts[1] {
                    "on" => {
                        new_state = true;
                    }
                    "off" => {
                        new_state = false;
                    }
                    _ => {
                        unimplemented!();
                    }
                }
                let points = parse_points(parts[2], parts[4]);
                (points[0].0..=points[0].1).for_each(|x| {
                    (points[1].0..=points[1].1).for_each(|y| {
                        lights[x][y] = new_state;
                    });
                });
            }
            "toggle" => {
                let points = parse_points(parts[1], parts[3]);
                (points[0].0..=points[0].1).for_each(|x| {
                    (points[1].0..=points[1].1).for_each(|y| {
                        lights[x][y] = !lights[x][y];
                    });
                });
            }
            _ => {}
        };
    }
    let mut lit_counter = 0;
    lights.iter().for_each(|row| {
        row.iter().for_each(|&light| {
            if light {
                lit_counter += 1;
            }
        })
    });
    lit_counter
}

fn part_two(path: &str) -> usize {
    let mut lights = vec![vec![0_usize; 1000]; 1000];
    let file: File = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let parts = string.split_whitespace().collect::<Vec<&str>>();
        let new_state;
        match parts[0] {
            "turn" => {
                match parts[1] {
                    "on" => {
                        new_state = true;
                    }
                    "off" => {
                        new_state = false;
                    }
                    _ => {
                        unimplemented!();
                    }
                }
                let points = parse_points(parts[2], parts[4]);
                (points[0].0..=points[0].1).for_each(|x| {
                    (points[1].0..=points[1].1).for_each(|y| {
                        if new_state {
                            lights[x][y] = lights[x][y].saturating_add(1);
                        } else {
                            lights[x][y] = lights[x][y].saturating_sub(1);
                        }
                    });
                });
            }
            "toggle" => {
                let points = parse_points(parts[1], parts[3]);
                (points[0].0..=points[0].1).for_each(|x| {
                    (points[1].0..=points[1].1).for_each(|y| {
                        lights[x][y] = lights[x][y].saturating_add(2);
                    });
                });
            }
            _ => {}
        };
    }
    let lit_counter = lights
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>();
    lit_counter
}

fn parse_points(lu_point: &str, rb_point: &str) -> Vec<(usize, usize)> {
    lu_point
        .split(',')
        .zip(rb_point.split(','))
        .map(|part| {
            (
                part.0.parse::<usize>().unwrap(),
                part.1.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}
