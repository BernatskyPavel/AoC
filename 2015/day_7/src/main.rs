use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let a = part_one(None);
    println!("{}", a);
    println!("{}", part_one(Some(a)));
}

#[derive(Copy, Clone, PartialEq)]
enum Operations {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Move
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
        let mut b_src: (u16, u16) = (0,0);
        match self.operation {
            Operations::Not | Operations::Move => {
                let value = self.src[0].parse::<u16>();
                if !value.is_err() {
                    u_src = value.unwrap();
                } else {
                    if registers.contains_key(&self.src[0]) {
                        u_src = *registers.get(&self.src[0]).unwrap();
                    } else {
                        return None;
                    }
                }
            },
            _ => {
                let (digits, regs) : (Vec<&String>, Vec<&String>) = self.src.iter().partition(|src|{
                    !src.parse::<u16>().is_err()
                });
                if digits.len() == 2 {
                    b_src = (digits[0].parse::<u16>().unwrap(), digits[1].parse::<u16>().unwrap());
                } else if digits.len() != 0 {
                    if regs.iter().all(|src|{
                        registers.contains_key(*src)
                    }) {
                        if registers.contains_key(regs[0]) {
                            if self.src[0] == *digits[0] {
                                b_src = (digits[0].parse::<u16>().unwrap(), *registers.get(regs[0]).unwrap());
                            } else {
                                b_src = (*registers.get(regs[0]).unwrap(), digits[0].parse::<u16>().unwrap());
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    if self.src.iter().all(|src|{
                        registers.contains_key(src)
                    }) {
                        b_src = (*registers.get(&self.src[0]).unwrap(), *registers.get(&self.src[1]).unwrap(),);
                    } else {
                        return None;
                    }
                }

            },
        };

        let mut result: (String, u16);

        match self.operation {
            Operations::And => {
                result = (self.dest.clone(), b_src.0 & b_src.1);
            },
            Operations::Or => {
                result = (self.dest.clone(), b_src.0 | b_src.1);
            },
            Operations::Lshift => {
                result = (self.dest.clone(), b_src.0 << b_src.1);
            },
            Operations::Rshift => {
                result = (self.dest.clone(), b_src.0 >> b_src.1);
            },
            Operations::Not => {
                result = (self.dest.clone(), !u_src);
            },
            Operations::Move => {
                result = (self.dest.clone(), u_src);
            }
        }
        if self.operation == Operations::Move && registers.get(&self.dest).is_some() {
            result.1 = *registers.get(&self.dest).unwrap();
        }
        Some(result)
    }
}

fn part_one(b: Option<u16>) -> u16 {

    lazy_static! {
        static ref BINARY: Regex = Regex::new("(?P<left>[a-z0-9]+) (?P<op>OR|AND|RSHIFT|LSHIFT) (?P<right>[a-z0-9]+) -> (?P<dest>[a-z0-9]+)").unwrap();
        static ref UNARY: Regex = Regex::new("^(?P<op>NOT )*(?P<src>[a-z0-9]+) -> (?P<dest>[a-z0-9]+)").unwrap();
    }
   
    let file: File = File::open("input.txt").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();  
    for line in BufReader::new(file).lines() {
        let string = line.unwrap().clone();
        let mut op_type = 0b000;
        let mut parts = BINARY.captures(string.as_str());
        if !parts.is_none() {
            op_type = 0b010;
        } else {
            parts = UNARY.captures(string.as_str());
            if !parts.is_none() {
                op_type = 0b001;
            }
        }

        let parts = parts.unwrap();
        let dest = String::from(parts.name("dest").unwrap().as_str());
        let operation_str = parts.name("op");
        let operation: Operations;
        if operation_str.is_none() {
            operation = Operations::Move;  
        } else {
            operation = match operation_str.unwrap().as_str().trim() {
                "NOT" => Operations::Not,
                "AND" => Operations::And,
                "OR" => Operations::Or,
                "LSHIFT" => Operations::Lshift,
                "RSHIFT" => Operations::Rshift,
                _ => unimplemented!()
            };
        };
        let mut src: Vec<String> = Vec::new();
        match op_type {
            0b001 => {    
                src.push(String::from(parts.name("src").unwrap().as_str()));
            },
            0b010 => {
                src.push(String::from(parts.name("left").unwrap().as_str()));
                src.push(String::from(parts.name("right").unwrap().as_str()));
            },
            _ => {
                unimplemented!()
            }
        };

        instructions.push(Instruction::new(operation, src, dest));
    }

    let mut blocked_counter = 0;
    let mut registers: BTreeMap<String, u16> = BTreeMap::new();
    if b.is_some() {
        registers.insert(String::from("b"), b.unwrap());
    }
    loop{
        let mut i = 0;
        loop {
            let result = instructions[i].execute(&registers);
            if !result.is_none() {
                let temp = result.unwrap();
                registers.insert(temp.0.clone(), temp.1);
                instructions.remove(i);
            } else {
                i+=1;
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