use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}

fn part_one() {
    let mut lights = vec![vec![false; 1000]; 1000];
    let file: File = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let parts = string.split_whitespace().collect::<Vec<&str>>();
        let new_state;
        match parts[0] {
            "turn" => {
                match parts[1] {
                    "on" => {
                        new_state = true;
                    },
                    "off" => {
                        new_state = false;
                    },
                    _ => {
                        unimplemented!();
                    },
                }
                let mut lu_point = (0,0);
                let mut rb_point = (0,0);
                let lu_coords = parts[2].split(',').collect::<Vec<&str>>();
                lu_point.0 = lu_coords[0].parse::<usize>().unwrap();
                lu_point.1 = lu_coords[1].parse::<usize>().unwrap();
                let rb_coords = parts[4].split(',').collect::<Vec<&str>>();
                rb_point.0 = rb_coords[0].parse::<usize>().unwrap();
                rb_point.1 = rb_coords[1].parse::<usize>().unwrap();
                (lu_point.0..=rb_point.0).for_each(|x|{
                    (lu_point.1..=rb_point.1).for_each(|y|{
                        lights[x][y] = new_state;
                    });
                });
            },
            "toggle" => {
                let mut lu_point = (0,0);
                let mut rb_point = (0,0);
                let lu_coords = parts[1].split(',').collect::<Vec<&str>>();
                lu_point.0 = lu_coords[0].parse::<usize>().unwrap();
                lu_point.1 = lu_coords[1].parse::<usize>().unwrap();
                let rb_coords = parts[3].split(',').collect::<Vec<&str>>();
                rb_point.0 = rb_coords[0].parse::<usize>().unwrap();
                rb_point.1 = rb_coords[1].parse::<usize>().unwrap();
                (lu_point.0..=rb_point.0).for_each(|x|{
                    (lu_point.1..=rb_point.1).for_each(|y|{
                        lights[x][y] = !lights[x][y];
                    });
                });
            },
            _ => {},
        };
    }
    let mut lit_counter = 0;
    lights.iter().for_each(|row|{
        row.iter().for_each(|&light|{
            if light {
                lit_counter += 1;
            }
        })
    });
    println!("{}", lit_counter);
}

fn part_two() {
    let mut lights = vec![vec![0_usize; 1000]; 1000];
    let file: File = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let parts = string.split_whitespace().collect::<Vec<&str>>();
        let new_state;
        match parts[0] {
            "turn" => {
                match parts[1] {
                    "on" => {
                        new_state = true;
                    },
                    "off" => {
                        new_state = false;
                    },
                    _ => {
                        unimplemented!();
                    },
                }
                let mut lu_point = (0,0);
                let mut rb_point = (0,0);
                let lu_coords = parts[2].split(',').collect::<Vec<&str>>();
                lu_point.0 = lu_coords[0].parse::<usize>().unwrap();
                lu_point.1 = lu_coords[1].parse::<usize>().unwrap();
                let rb_coords = parts[4].split(',').collect::<Vec<&str>>();
                rb_point.0 = rb_coords[0].parse::<usize>().unwrap();
                rb_point.1 = rb_coords[1].parse::<usize>().unwrap();
                (lu_point.0..=rb_point.0).for_each(|x|{
                    (lu_point.1..=rb_point.1).for_each(|y|{
                        if new_state {
                            lights[x][y] = lights[x][y].saturating_add(1);
                        } else {
                            lights[x][y] = lights[x][y].saturating_sub(1);
                        }
                        
                    });
                });
            },
            "toggle" => {
                let mut lu_point = (0,0);
                let mut rb_point = (0,0);
                let lu_coords = parts[1].split(',').collect::<Vec<&str>>();
                lu_point.0 = lu_coords[0].parse::<usize>().unwrap();
                lu_point.1 = lu_coords[1].parse::<usize>().unwrap();
                let rb_coords = parts[3].split(',').collect::<Vec<&str>>();
                rb_point.0 = rb_coords[0].parse::<usize>().unwrap();
                rb_point.1 = rb_coords[1].parse::<usize>().unwrap();
                (lu_point.0..=rb_point.0).for_each(|x|{
                    (lu_point.1..=rb_point.1).for_each(|y|{
                        lights[x][y] = lights[x][y].saturating_add(2);
                    });
                });
            },
            _ => {},
        };
    }
    let lit_counter =
    lights.iter().map(|row|{
        row.iter().sum::<usize>()
    }).sum::<usize>();
    println!("{}", lit_counter);
}