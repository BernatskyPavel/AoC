use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    let a = part_one("input.txt", None);
    println!(
        "Part one: {}\nPart two: {}",
        a,
        part_one("input.txt", Some(a))
    );
}

#[derive(Copy, Clone, PartialEq)]
enum Operations {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Move,
}

#[derive(Clone)]
struct Instruction {
    src: Vec<String>,
    dest: String,
    operation: Operations,
}

impl Instruction {
    fn new(operation: Operations, src: Vec<String>, dest: String) -> Instruction {
        Instruction {
            src,
            dest,
            operation,
        }
    }

    fn execute(&self, registers: &BTreeMap<String, u16>) -> Option<(String, u16)> {
        let mut u_src: u16 = 0;
        let mut b_src: (u16, u16) = (0, 0);
        match self.operation {
            Operations::Not | Operations::Move => {
                if let Ok(val) = self.src[0].parse::<u16>() {
                    u_src = val;
                } else if registers.contains_key(&self.src[0]) {
                    u_src = *registers.get(&self.src[0]).unwrap();
                } else {
                    return None;
                }
            }
            _ => {
                let (digits, regs): (Vec<&String>, Vec<&String>) =
                    self.src.iter().partition(|src| src.parse::<u16>().is_ok());

                if digits.len() == 2 {
                    b_src = (
                        digits[0].parse::<u16>().unwrap(),
                        digits[1].parse::<u16>().unwrap(),
                    );
                } else if !digits.is_empty() {
                    if regs.iter().all(|src| registers.contains_key(*src)) {
                        if self.src[0] == *digits[0] {
                            b_src = (
                                digits[0].parse::<u16>().unwrap(),
                                *registers.get(regs[0]).unwrap(),
                            );
                        } else {
                            b_src = (
                                *registers.get(regs[0]).unwrap(),
                                digits[0].parse::<u16>().unwrap(),
                            );
                        }
                    } else {
                        return None;
                    }
                } else if self.src.iter().all(|src| registers.contains_key(src)) {
                    b_src = (
                        *registers.get(&self.src[0]).unwrap(),
                        *registers.get(&self.src[1]).unwrap(),
                    );
                } else {
                    return None;
                }
            }
        };

        let mut result: (String, u16) = match self.operation {
            Operations::And => (self.dest.clone(), b_src.0 & b_src.1),
            Operations::Or => (self.dest.clone(), b_src.0 | b_src.1),
            Operations::Lshift => (self.dest.clone(), b_src.0 << b_src.1),
            Operations::Rshift => (self.dest.clone(), b_src.0 >> b_src.1),
            Operations::Not => (self.dest.clone(), !u_src),
            Operations::Move => (self.dest.clone(), u_src),
        };

        if self.operation == Operations::Move && registers.get(&self.dest).is_some() {
            result.1 = *registers.get(&self.dest).unwrap();
        }
        Some(result)
    }
}

fn part_one(path: &str, b: Option<u16>) -> u16 {
    lazy_static! {
        static ref BINARY: Regex = Regex::new("(?P<left>[a-z0-9]+) (?P<op>OR|AND|RSHIFT|LSHIFT) (?P<right>[a-z0-9]+) -> (?P<dest>[a-z0-9]+)").unwrap();
        static ref UNARY: Regex = Regex::new("^(?P<op>NOT )*(?P<src>[a-z0-9]+) -> (?P<dest>[a-z0-9]+)").unwrap();
    }

    let file: File = File::open(path).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in BufReader::new(file).lines() {
        let string = line.unwrap().clone();
        let (op_type, parts) = if let Some(val) = BINARY.captures(string.as_str()) {
            (0b010, val)
        } else if let Some(val) = UNARY.captures(string.as_str()) {
            (0b001, val)
        } else {
            unimplemented!()
        };

        let dest = String::from(parts.name("dest").unwrap().as_str());
        let operation_str = parts.name("op");
        let operation: Operations = if let Some(op) = operation_str {
            match op.as_str().trim() {
                "NOT" => Operations::Not,
                "AND" => Operations::And,
                "OR" => Operations::Or,
                "LSHIFT" => Operations::Lshift,
                "RSHIFT" => Operations::Rshift,
                _ => unimplemented!(),
            }
        } else {
            Operations::Move
        };

        let mut src: Vec<String> = Vec::new();
        match op_type {
            0b001 => {
                src.push(String::from(parts.name("src").unwrap().as_str()));
            }
            0b010 => {
                src.push(String::from(parts.name("left").unwrap().as_str()));
                src.push(String::from(parts.name("right").unwrap().as_str()));
            }
            _ => {
                unimplemented!()
            }
        };

        instructions.push(Instruction::new(operation, src, dest));
    }

    let mut blocked_counter = 0;
    let mut registers: BTreeMap<String, u16> = BTreeMap::new();

    if let Some(val) = b {
        registers.insert(String::from("b"), val);
    }
    loop {
        let mut i = 0;
        loop {
            if let Some(val) = instructions[i].execute(&registers) {
                registers.insert(val.0.clone(), val.1);
                instructions.remove(i);
            } else {
                i += 1;
                blocked_counter += 1;
            }
            if i == instructions.len() {
                break;
            }
        }
        if blocked_counter == 0 {
            break;
        }
        blocked_counter = 0;
    }
    *registers.get("a").unwrap()
}
