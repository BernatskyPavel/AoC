use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let sum_all = part_one();
    println!("{}", sum_all);
    println!("{}", sum_all - part_two());
}

fn part_one() -> isize {
    let file: File = File::open("input.txt").unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new("(?P<number>[-]*[1-9][0-9]*)").unwrap();
    }
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            sum += cap["number"].parse::<isize>().unwrap();
        });
    }
    sum
}

fn part_two() -> isize {
    let file: File = File::open("input.txt").unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new("(?P<number>[-]*[1-9][0-9]*)").unwrap();
        static ref RED: Regex = Regex::new(":\"red\"").unwrap();
    }
    let mut open_counter = 0;
    let mut objects_buf: Vec<(usize, usize)> = Vec::new();
    let mut objects: Vec<(usize, usize)> = Vec::new();
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let mut string = line.unwrap();
        string.push('$');
        string.chars().enumerate().for_each(|ch| match ch.1 {
            '{' => {
                open_counter += 1;
                objects_buf.push((ch.0, 0));
            }
            '}' => {
                objects_buf[open_counter - 1].1 = ch.0;
                objects.push(objects_buf.pop().unwrap());
                open_counter -= 1;
            }
            _ => {}
        });
        if objects.len() > 0 {
           // println!("{:?}", objects);
            let mut blocked_objects: Vec<(usize, usize)> = Vec::new();
            RED.find_iter(string.as_str()).for_each(|red| {
                //println!("{:?}-{}", red.start(), red.end());
                let mut temp = objects
                    .iter()
                    .filter(|object| red.start() > object.0 && red.end() <= object.1)
                    .collect::<Vec<&(usize, usize)>>();
                if temp.len() > 1 {
                    temp.sort_by(|a, b| (red.start().saturating_sub(a.0)).partial_cmp(&(red.start().saturating_sub(b.0))).unwrap());
                }
                if !blocked_objects
                    .iter()
                    .any(|object| object.0 <= temp[0].0 && object.1 >= temp[0].1)
                {
                    if blocked_objects
                        .iter()
                        .any(|object| object.0 > temp[0].0 && object.1 < temp[0].1)
                    {
                        blocked_objects = blocked_objects
                            .iter()
                            .cloned()
                            .filter(|object| !(object.0 > temp[0].0 && object.1 < temp[0].1))
                            .collect();
                    }
                    blocked_objects.push(*temp[0]);
                }

               // println!("{:?}", temp[0]);
            });
           // println!("{:?}", blocked_objects);

            blocked_objects.iter().for_each(|object| {
                let object_string = string.get(object.0..=object.1).unwrap();
                //println!("{:?}", object_string);
                RE.captures_iter(object_string).for_each(|cap| {
                    sum += cap["number"].parse::<isize>().unwrap();
                });
            });
        }
    }
    sum
}
