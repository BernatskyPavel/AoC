use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let seats = get_seats_list("input.txt");
    println!("{}", part_one(&seats));
    println!("{}", part_two(&seats));
}

fn part_one(seats_list: &Vec<usize>) -> usize{
    *seats_list.iter().max().unwrap()
}

fn part_two(seats_list: &Vec<usize>) -> usize {
    let current = seats_list.windows(2).take_while(|pair| {
        (**pair)[0] == (**pair)[1] - 1
    });
    current.last().unwrap()[1] + 1
}

fn get_seats_list(input: &str) -> Vec<usize> {
    let file: File = File::open(input).unwrap();
    let mut seats: Vec<usize> = Vec::new();
    for line in BufReader::new(file).lines() {
        let mut row = String::new();
        let mut seat = String::new();
        line.unwrap().chars().for_each(|ch| {
            match ch {
                'F' => row.push('0'),
                'B' => row.push('1'),
                'L' => seat.push('0'),
                'R' => seat.push('1'),
                _ => unimplemented!(),
            }
        });
        seats.push(usize::from_str_radix(row.as_str(), 2).unwrap() * 8 + usize::from_str_radix(seat.as_str(), 2).unwrap());
    };
    seats.sort();
    seats
}