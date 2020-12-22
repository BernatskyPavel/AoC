use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    println!("{}", part_two());
}

fn part_one() -> isize {
    let mut adapters: Vec<isize> = Vec::new();
    {
        let file: File = File::open("input.txt").unwrap();

        for line in BufReader::new(file).lines() {
            let string = line.unwrap();
            adapters.push(string.parse::<isize>().unwrap());
        }
    }
    let mut curretnt_adapter = 0;
    let (mut ones, mut threes) = (0, 1);
    adapters.sort();
    loop {
        let next = vec![
            curretnt_adapter + 1,
            curretnt_adapter + 2,
            curretnt_adapter + 3,
        ];
        let temp = adapters
            .iter()
            .filter(|&adapter| next.contains(adapter))
            .collect::<Vec<&isize>>();
        if temp.len() == 0 {
            break;
        }
        match temp[0] - curretnt_adapter {
            1 => {
                ones += 1;
            }
            3 => {
                threes += 1;
            }
            _ => {}
        }
        curretnt_adapter = *temp[0];
    }
    ones * threes
}

fn part_two() -> isize {
    let mut adapters: Vec<isize> = Vec::new();
    {
        let file: File = File::open("input.txt").unwrap();

        for line in BufReader::new(file).lines() {
            let string = line.unwrap();
            adapters.push(string.parse::<isize>().unwrap());
        }
    }
    let mut chains: Vec<isize> = Vec::new();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let mut i = 0;
    loop {
        let mut j = i + 1;
        let mut chain = 0;
        loop {
            if adapters[j] - adapters[i] <= 3 {
                chain += 1;
                j += 1;
                if j == adapters.len() {
                    chains.push(chain);
                    break;
                }
            } else {
                chains.push(chain);
                break;
            }
        }

        i += 1;
        if i == adapters.len() - 1 {
            break;
        }
    }
    println!("{:?}", adapters);
    let mut src = chains
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join("");
    src = src.replace("33211", "7");
    src = src.replace("3211", "4");
    src.chars()
        .map(|ch| ch.to_string().parse::<isize>().unwrap())
        .product::<isize>()
}
