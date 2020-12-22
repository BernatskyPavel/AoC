use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let data = extract_from_file("input.txt");
    println!("{}", part_one(data.0, &data.1));
    println!("{}", part_two(&data.1));
}

fn part_one(timestamp: usize, buses: &Vec<(usize, usize)>) -> usize {
    let bus = buses
        .clone()
        .iter()
        .map(|bus_id| {
            let mut buff = bus_id.1;
            loop {
                buff += bus_id.1;
                if buff >= timestamp {
                    break (bus_id.1, buff - timestamp);
                }
            }
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    bus.1 * bus.0
}

fn part_two(buses: &Vec<(usize, usize)>) -> usize {
    //println!("{:?}", buses);
    let mut buf = buses[0];
    let mut step = buf.1;
    let mut i = 2;
    loop {
        loop {
            buf.1 += step;
            if buses[0..i]
                .iter()
                .all(|bus| (buf.1 + bus.0 - buf.0) % bus.1 == 0)
            {
                step = buses[0..i].iter().map(|bus| bus.1).product();
                break;
            }
        }
        if i == buses.len() {
            break;
        }
        i += 1;
    }
    buf.1 - buf.0
}

fn extract_from_file(file: &str) -> (usize, Vec<(usize, usize)>) {
    let file: File = File::open(file).unwrap();
    let mut error_counter = 0;
    let mut timestamp: usize = 0;
    let mut buses: Vec<(usize, usize)> = Vec::new();
    for line in BufReader::new(file).lines().enumerate() {
        let string = line.1.unwrap();
        match line.0 {
            0 => {
                let option_timestamp = string.parse::<usize>();
                if option_timestamp.is_err() {
                    error_counter += 1;
                    continue;
                } else {
                    timestamp = option_timestamp.unwrap();
                }
            }
            1 => {
                buses = string
                    .split(",")
                    .enumerate()
                    .filter(|bus| bus.1.parse::<usize>().is_ok())
                    .map(|bus| (bus.0, bus.1.parse::<usize>().unwrap()))
                    .collect();
            }
            _ => unimplemented!(),
        }
    }
    if error_counter != 0 {
        println!(
            "Errors: {}! Correct result is not guaranteed!",
            error_counter
        );
    } else {
        println!("File parsed with no errors!");
    };
    (timestamp, buses)
}
