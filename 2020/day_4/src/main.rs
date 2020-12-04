use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let mut valid_passes = part_one();
    println!("{}", valid_passes.len());
    part_two(&mut valid_passes);
    println!("{}", valid_passes.len());
}

fn part_one() -> Vec<BTreeMap<String, String>> {
    let file: File = File::open("input.txt").unwrap();
    let required_fields = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];
    let all_fields = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
        String::from("cid"),
    ];
    let mut passports: BTreeMap<String, String> = BTreeMap::new();
    let mut valid_passports: Vec<BTreeMap<String, String>> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap();
        if string.is_empty() {
            let keys = passports.keys().collect::<Vec<&String>>();
            let is_all_presented = all_fields.iter().all(|key| keys.contains(&key));
            if is_all_presented {
                valid_passports.push(passports.clone());
            } else {
                if required_fields.iter().all(|key| keys.contains(&key)) {
                    valid_passports.push(passports.clone());
                }
            }
            passports.clear();
        } else {
            string.split_whitespace().for_each(|fields| {
                let values = fields.split(":").collect::<Vec<&str>>();
                passports.insert(String::from(values[0]), String::from(values[1]));
            });
        }
    }
    valid_passports
}

fn part_two(passports: &mut Vec<BTreeMap<String, String>>) {
    passports.retain(|passport| {
        let mut is_valid = true;
        passport.iter().for_each(|fields| match fields.0.as_str() {
            "byr" => {
                let temp = fields.1.parse::<usize>();
                if temp.is_err() {
                    is_valid = false;
                } else {
                    let byr = temp.unwrap();
                    if byr < 1920 || byr > 2002 {
                        is_valid = false;
                    }
                }
            }
            "iyr" => {
                let temp = fields.1.parse::<usize>();
                if temp.is_err() {
                    is_valid = false;
                } else {
                    let iyr = temp.unwrap();
                    if iyr < 2010 || iyr > 2020 {
                        is_valid = false;
                    }
                }
            }
            "eyr" => {
                let temp = fields.1.parse::<usize>();
                if temp.is_err() {
                    is_valid = false;
                } else {
                    let eyr = temp.unwrap();
                    if eyr < 2020 || eyr > 2030 {
                        is_valid = false;
                    }
                }
            }
            "hgt" => {
                let mut measurments = fields.1.clone();
                measurments.retain(|ch| !ch.is_numeric());
                let mut height = fields.1.clone();
                height.retain(|ch| ch.is_numeric());
                let height = height.parse::<usize>();
                if height.is_err() {
                    is_valid = false;
                } else {
                    let height = height.unwrap();
                    match measurments.as_str() {
                        "cm" => {
                            if height < 150 || height > 193 {
                                is_valid = false;
                            }
                        }
                        "in" => {
                            if height < 59 || height > 76 {
                                is_valid = false;
                            }
                        }
                        _ => {
                            is_valid = false;
                        }
                    }
                }
            }
            "hcl" => {
                let hcl = fields.1.clone();
                let hex_color = hcl.strip_prefix("#");
                if hex_color.is_none() {
                    is_valid = false;
                } else {
                    let hex_color = hex_color.unwrap();
                    if hex_color.len() != 6 || !hex_color.chars().all(|ch| ch.is_ascii_hexdigit()) {
                        is_valid = false;
                    }
                }
            }
            "ecl" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&fields.1.as_str()) {
                    is_valid = false;
                }
            }
            "pid" => {
                let pid = fields.1.parse::<usize>();
                if pid.is_err() {
                    is_valid = false;
                } else {
                    if fields.1.len() != 9 {
                        is_valid = false;
                    }
                }
            }
            _ => {}
        });
        is_valid
    });
}
