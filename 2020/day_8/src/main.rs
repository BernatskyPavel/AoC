use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    println!("{}", part_two());
}

fn part_one() -> isize {
    lazy_static! {
        static ref CMD: Regex = Regex::new("(?P<cmd>nop|acc|jmp) (?P<arg>[+|-][0-9]+)").unwrap();
    }
    let file: File = File::open("input.txt").unwrap();
    let mut boot: Vec<(String, isize, bool)> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let code = CMD.captures(string.as_str()).unwrap();
        boot.push((
            String::from(code.name("cmd").unwrap().as_str()),
            code.name("arg").unwrap().as_str().parse::<isize>().unwrap(),
            false,
        ));
    }
    //println!("{:?}", boot);
    let mut acc = 0;
    let mut i = 0;
    loop {
        //println!("1--{}-{}-{:?}", i, acc, boot[i]);
        if boot[i].2 {
            break;
        }
        boot[i].2 = true;
        match boot[i].0.as_str() {
            "acc" => {
                acc += boot[i].1;
                i += 1;
            }
            "jmp" => {
                if boot[i].1 < 0 {
                    i = i.saturating_sub(boot[i].1.abs() as usize);
                } else {
                    i = i.saturating_add(boot[i].1.abs() as usize);
                }
            }
            "nop" => {
                i += 1;
            }
            _ => unreachable!(),
        }
        //println!("2--{}-{}-{:?}", i, acc, boot[i]);
    }
    acc
}

fn part_two() -> isize {
    lazy_static! {
        static ref CMD: Regex = Regex::new("(?P<cmd>nop|acc|jmp) (?P<arg>[+|-][0-9]+)").unwrap();
    }
    let file: File = File::open("input.txt").unwrap();
    let mut original_boot: Vec<(String, isize, bool)> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        let code = CMD.captures(string.as_str()).unwrap();
        original_boot.push((
            String::from(code.name("cmd").unwrap().as_str()),
            code.name("arg").unwrap().as_str().parse::<isize>().unwrap(),
            false,
        ));
    }
    //println!("{:?}", boot);
    let mut variants: Vec<usize> = original_boot.iter().enumerate().filter(|cmd|{
        ["nop", "jmp"].contains(&cmd.1.0.as_str()) 
    }).map(|cmd|{
        cmd.0
    }).collect();
    let mut acc = 0;
    let mut is_finished = false;
    loop {
        let mut boot = original_boot.clone();
        let temp = variants.pop().unwrap();
        boot[temp].0 = match boot[temp].0.as_str() {
            "nop" => { String::from("jmp") },
            "jmp" => { String::from("nop") },
            _ => unreachable!()
        };
        let mut i = 0;
        loop {
            //println!("1--{}-{}-{:?}", i, acc, boot[i]);
            if boot[i].2 {
                break;
            }
            boot[i].2 = true;
            match boot[i].0.as_str() {
                "acc" => {
                    acc += boot[i].1;
                    i += 1;
                }
                "jmp" => {
                    if boot[i].1 < 0 {
                        i = i.saturating_sub(boot[i].1.abs() as usize);
                    } else {
                        i = i.saturating_add(boot[i].1.abs() as usize);
                    }
                }
                "nop" => {
                    i += 1;
                }
                _ => unreachable!(),
            }
            if i == boot.len() {
                is_finished = true;
                break;
            }
            //println!("2--{}-{}-{:?}", i, acc, boot[i]);
        }
        if is_finished {
            break;
        } else {
            acc = 0;
        }
    }
    acc
}
