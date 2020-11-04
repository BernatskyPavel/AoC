fn main() {
    part_one();
    //part_two_2();
    //println!("Hello, world!");
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() {
    let map = form_and_weigh_2("file.txt");
    println!("{:?}", map.values().map(|value|{
        value.0
    }).sum::<u32>()); 
}

fn part_two() {
    let mut map = form_and_weigh("file.txt");
    // loop {
    //     if me_step.eq(&String::from("COM"));
    // };
}

fn part_two_2() {
    let mut map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let file: File = File::open("file.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let temp = line
            .unwrap()
            .split(")")
            .map(|x| x.clone().to_string())
            .collect::<Vec<String>>();
        let entry = map.entry(temp[0].clone()).or_insert(Vec::new());
        (*entry).push((temp[1].clone(), 0));
    }
    let mut orbits = 1;
    let com = map.entry(String::from("COM")).or_default();
    let mut next_step = (*com).clone();
    for node in com {
        node.1 = orbits;
    }
    loop {
        let mut temp: Vec<(String, u32)> = Vec::new();
        orbits += 1;
        next_step.iter().for_each(|x| {
            let entry = map.entry(x.0.clone()).or_default();
            for step in entry {
                step.1 = orbits;
                temp.push(step.clone());
            }
        });
        next_step.clear();
        next_step.append(&mut temp);
        if next_step.is_empty() {
            break;
        }
    }
    println!("{:?}", map.values().map(|value|{
        value.iter().map(|sub|{
            sub.1
        }).sum::<u32>()
    }).sum::<u32>());    
}

fn form_and_weigh(path: &str) -> HashMap<String, Vec<(String, u32)>> {
    let mut map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let file: File = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let temp = line
            .unwrap()
            .split(")")
            .map(|x| x.clone().to_string())
            .collect::<Vec<String>>();
        let entry = map.entry(temp[0].clone()).or_insert(Vec::new());
        (*entry).push((temp[1].clone(), 0));
    }
    let mut orbits = 1;
    let com = map.entry(String::from("COM")).or_default();
    let mut next_step = (*com).clone();
    for node in com {
        node.1 = orbits;
    }
    loop {
        let mut temp: Vec<(String, u32)> = Vec::new();
        orbits += 1;
        next_step.iter().for_each(|x| {
            let entry = map.entry(x.0.clone()).or_default();
            for step in entry {
                step.1 = orbits;
                temp.push(step.clone());
            }
        });
        next_step.clear();
        next_step.append(&mut temp);
        if next_step.is_empty() {
            break;
        }
    }
    map  
}

fn form_and_weigh_2(path: &str) -> HashMap<String, (u32, Vec<String>)> {
    let mut map: HashMap<String, (u32, Vec<String>)> = HashMap::new();
    let file: File = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let temp = line
            .unwrap()
            .split(")")
            .map(|x| x.clone().to_string())
            .collect::<Vec<String>>();
        let entry = map.entry(temp[0].clone()).or_insert((0, Vec::new()));
        (*entry).1.push(temp[1].clone());
        (*entry).0 = 0;
    }
    let mut orbits = 1;
    let com = map.entry(String::from("COM")).or_default();
    let mut next_step = (*com).clone();
    loop {
        let mut temp: Vec<String> = Vec::new();
        next_step.0 = orbits;
        next_step.1.iter().for_each(|x| {
            let entry = map.entry(x.clone()).or_default();
            entry.0 = orbits;
            for step in entry.1.clone() {
                temp.push(step.clone());
            }
        });
        orbits += 1;
        next_step.1.clear();
        next_step.1.append(&mut temp);
        if next_step.1.is_empty() {
            break;
        }
    }
    println!("{:?}", map);
    map  
}