use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Part one: {}", part_one("input.txt", 100));
    println!("Part two: {}", part_two("input.txt", 100, 500));
}

struct Ingridient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Ingridient {
    fn new(
        capacity: isize,
        durability: isize,
        flavor: isize,
        texture: isize,
        calories: isize,
    ) -> Self {
        Ingridient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn part_one(file: &str, max_spoons: usize) -> isize {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<ingridient>[A-Za-z]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
    }

    let mut ingridients: Vec<Ingridient> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            ingridients.push(Ingridient::new(
                cap["capacity"].parse::<isize>().unwrap(),
                cap["durability"].parse::<isize>().unwrap(),
                cap["flavor"].parse::<isize>().unwrap(),
                cap["texture"].parse::<isize>().unwrap(),
                cap["calories"].parse::<isize>().unwrap(),
            ));
        });
    }

    let mut max = 0;
    let mut spoons = vec![0; ingridients.len()];
    let index = ingridients.len() - 1;
    let limit = max_spoons + 1;
    loop {
        if spoons.iter().sum::<usize>() == max_spoons {
            let (mut cap, mut dur, mut fl, mut tex) = (0, 0, 0, 0);
            (0..=index).for_each(|i| {
                let ingridient = &ingridients[i];
                cap += ingridient.capacity * spoons[i] as isize;
                dur += ingridient.durability * spoons[i] as isize;
                fl += ingridient.flavor * spoons[i] as isize;
                tex += ingridient.texture * spoons[i] as isize;
            });
            let result = cap.max(0) * dur.max(0) * fl.max(0) * tex.max(0);
            max = max.max(result);
        }
        let mut prev = (spoons[index] + 1) % limit;
        spoons[index] = prev;
        (0..index).rev().for_each(|i| {
            if prev == 0 {
                spoons[i] += 1;
                spoons[i + 1] = prev;
            }
            prev = (spoons[i] + 1) % limit;
        });
        if spoons[0] == max_spoons {
            break;
        }
    }

    max
}

fn part_two(file: &str, max_spoons: usize, calories: isize) -> isize {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<ingridient>[A-Za-z]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
    }

    let mut ingridients: Vec<Ingridient> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            ingridients.push(Ingridient::new(
                cap["capacity"].parse::<isize>().unwrap(),
                cap["durability"].parse::<isize>().unwrap(),
                cap["flavor"].parse::<isize>().unwrap(),
                cap["texture"].parse::<isize>().unwrap(),
                cap["calories"].parse::<isize>().unwrap(),
            ));
        });
    }

    let mut max = 0;
    let mut spoons = vec![0; ingridients.len()];
    let index = ingridients.len() - 1;
    let limit = max_spoons + 1;
    loop {
        if spoons.iter().sum::<usize>() == max_spoons {
            let (mut cap, mut dur, mut fl, mut tex, mut cal) = (0, 0, 0, 0, 0);
            (0..=index).for_each(|i| {
                let ingridient = &ingridients[i];
                cap += ingridient.capacity * spoons[i] as isize;
                dur += ingridient.durability * spoons[i] as isize;
                fl += ingridient.flavor * spoons[i] as isize;
                tex += ingridient.texture * spoons[i] as isize;
                cal += ingridient.calories * spoons[i] as isize;
            });
            let result = cap.max(0) * dur.max(0) * fl.max(0) * tex.max(0);
            if result > max && cal == calories {
                max = result;
            }
        }
        let mut prev = (spoons[index] + 1) % limit;
        spoons[index] = prev;
        (0..index).rev().for_each(|i| {
            if prev == 0 {
                spoons[i] += 1;
                spoons[i + 1] = prev;
            }
            prev = (spoons[i] + 1) % limit;
        });
        if spoons[0] == max_spoons {
            break;
        }
    }

    max
}
