#![allow(dead_code)]
fn main() {
    //part_one();
    part_two();
    //println!("Hello, world!");
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
 
fn part_one() {
    let map = form_and_weigh("file.txt");
    println!("{:?}", map.values().map(|value|{
        value.0
    }).sum::<u32>()); 
}

fn part_two() {
    let map = form_and_weigh("file.txt");
    let mut me_path: Vec<String> = Vec::new();
    let mut next_step = String::from("YOU");
    loop {
        next_step = map.iter().find(|&p|{
            p.1.1.contains(&next_step.clone())
        }).unwrap().0.clone();
        me_path.push(next_step.clone());
        if next_step.eq("COM") {
            break;
        }
    }
    let mut san_path: Vec<String> = Vec::new();
    next_step = String::from("SAN");
    loop {
        next_step = map.iter().find(|&p|{
            p.1.1.contains(&next_step.clone())
        }).unwrap().0.clone();
        san_path.push(next_step.clone());
        if next_step.eq("COM") {
            break;
        }
    }

    loop {
         if me_path.last() != san_path.last() {
             break;
         }
         me_path.pop();
         san_path.pop();
     }
    println!("{}", me_path.len() + san_path.len());
}

fn form_and_weigh(path: &str) -> HashMap<String, (u32, Vec<String>)> {
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
    map  
}