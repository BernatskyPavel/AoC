use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Part one: {}", part_one("input.txt", 2503));
    println!("Part two: {}", part_two("input.txt", 2503));
}
struct DeerInfo {
    speed: usize,
    run_time: usize,
    rest_time: usize,
}

impl DeerInfo {
    fn new(speed: usize, run_time: usize, rest_time: usize) -> Self {
        DeerInfo {
            speed,
            run_time,
            rest_time,
        }
    }

    fn run_for(&self, seconds: usize) -> usize {
        let iters = seconds / (self.rest_time + self.run_time);
        let time_remained = seconds - iters * (self.rest_time + self.run_time);
        if time_remained >= self.run_time {
            self.speed * self.run_time * (iters + 1)
        } else {
            self.speed * (self.run_time * iters + time_remained)
        }
    }
}

fn part_one(file: &str, seconds: usize) -> usize {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<deer>[A-Za-z]+) can fly (?P<speed>\d+) km/s for (?P<time>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.").unwrap();
    }

    let mut deers: Vec<DeerInfo> = Vec::new();

    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            deers.push(DeerInfo::new(
                cap["speed"].parse::<usize>().unwrap(),
                cap["time"].parse::<usize>().unwrap(),
                cap["rest"].parse::<usize>().unwrap(),
            ));
        });
    }
    deers
        .iter()
        .map(|deer| deer.run_for(seconds))
        .max()
        .unwrap()
}

struct PointCounter {
    deers: Vec<DeerInfo>,
    leader_points: usize,
    points: Vec<usize>,
}

impl PointCounter {
    fn new(deers: Vec<DeerInfo>) -> Self {
        let len = deers.len();
        PointCounter {
            deers,
            leader_points: 0,
            points: vec![0; len],
        }
    }

    fn calc_points_for(&mut self, seconds: usize) {
        (1..=seconds).for_each(|i| {
            let mut points: Vec<_> = self
                .deers
                .iter()
                .enumerate()
                .map(|deer| (deer.0, deer.1.run_for(i)))
                .collect();
            points.sort_by(|a, b| b.1.cmp(&a.1));
            self.points[points[0].0] += 1;
            points.iter().skip(1).for_each(|deer| {
                if deer.1 == points[0].1 {
                    self.points[deer.0] += 1;
                }
            });
        });
        self.leader_points = *self.points.iter().max().unwrap();
    }
}

fn part_two(file: &str, seconds: usize) -> usize {
    let file: File = File::open(file).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<deer>[A-Za-z]+) can fly (?P<speed>\d+) km/s for (?P<time>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.").unwrap();
    }

    let mut deers: Vec<DeerInfo> = Vec::new();

    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        RE.captures_iter(string.as_str()).for_each(|cap| {
            deers.push(DeerInfo::new(
                cap["speed"].parse::<usize>().unwrap(),
                cap["time"].parse::<usize>().unwrap(),
                cap["rest"].parse::<usize>().unwrap(),
            ));
        });
    }
    let mut race = PointCounter::new(deers);
    race.calc_points_for(seconds);
    race.leader_points
}
