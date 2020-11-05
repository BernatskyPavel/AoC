#![allow(dead_code)]
use itertools::Itertools;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread::Builder;

fn main() {
    part_one();
}

fn part_one() {
    let permutations_iterator = [0_isize, 1, 2, 3, 4].iter().permutations(5);
    let mut result: isize = 0;
    let mut input;
    for permutation in permutations_iterator {
        input = 0;
        for step in permutation {
            let init_1 = step;
            let (tx, rx_1) = mpsc::channel::<isize>();
            let (tx_1, rx) = mpsc::channel::<isize>();

            tx.send(input).expect("Send failed!");

            Builder::new()
                .name("amp_a".into())
                .spawn(move || intcode(rx_1, tx_1, init_1))
                .unwrap();

            input = rx.recv().unwrap();

            if input > result {
                result = input;
            }
        }
    }
    println!("{}", result);
}

fn part_two() {
    let permutations_iterator = [5_isize, 6, 7, 8, 9].iter().permutations(5);
    let mut result: isize = 0;

    for permutation in permutations_iterator {
        let (init_1, init_2, init_3, init_4, init_5) = (
            *(permutation.clone().iter().nth(0).unwrap()),
            *(permutation.clone().iter().nth(1).unwrap()),
            *(permutation.clone().iter().nth(2).unwrap()),
            *(permutation.clone().iter().nth(3).unwrap()),
            *(permutation.clone().iter().nth(4).unwrap()),
        );
        let (tx_1, rx_2) = mpsc::channel::<isize>();
        let (tx_2, rx_3) = mpsc::channel::<isize>();
        let (tx_3, rx_4) = mpsc::channel::<isize>();
        let (tx_4, rx_5) = mpsc::channel::<isize>();
        let (tx_5, rx_1) = mpsc::channel::<isize>();

        Builder::new()
            .name("amp_a".into())
            .spawn(move || intcode(rx_1, tx_1, init_1))
            .unwrap();
        &tx_5.send(0);
        Builder::new()
            .name("amp_b".into())
            .spawn(move || intcode(rx_2, tx_2, init_2))
            .unwrap();
        Builder::new()
            .name("amp_c".into())
            .spawn(move || intcode(rx_3, tx_3, init_3))
            .unwrap();
        Builder::new()
            .name("amp_d".into())
            .spawn(move || intcode(rx_4, tx_4, init_4))
            .unwrap();
        let amp_5 = Builder::new()
            .name("amp_e".into())
            .spawn(move || intcode(rx_5, tx_5, init_5))
            .unwrap();

        match amp_5.join().ok() {
            Some(value) => {
                if result < value {
                    result = value;
                }
            }
            None => {}
        }
    }
    println!("{}", result);
}

enum Opcode {
    Sum(usize),
    Product(usize),
    Input(usize),
    Output(usize),
    JumpT(usize),
    JumpF(usize),
    Less(usize),
    Equal(usize),
    Halt(usize),
}

fn intcode(rx: Receiver<isize>, tx: Sender<isize>, init: &isize) -> isize {
    let mut is_first = true;
    let mut codes = [
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 59, 72, 93, 110, 191, 272, 353, 434, 99999, 3,
        9, 101, 4, 9, 9, 1002, 9, 3, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9,
        1002, 9, 5, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9,
        102, 3, 9, 9, 101, 3, 9, 9, 1002, 9, 2, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9,
        102, 4, 9, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4,
        9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 99,
    ];
    let mut i = 0;
    loop {
        if i >= codes.len() {
            break;
        }
        let opcode = parse_opcode(codes[i]);

        let params = match opcode.0 {
            Opcode::Halt(_) => (0, 0, 0),
            Opcode::Sum(_) | Opcode::Product(_) | Opcode::Equal(_) | Opcode::Less(_) => (
                if opcode.1 == 0 {
                    codes[i + 1] as usize
                } else {
                    i + 1
                },
                if opcode.2 == 0 {
                    codes[i + 2] as usize
                } else {
                    i + 2
                },
                if opcode.3 == 0 {
                    codes[i + 3] as usize
                } else {
                    i + 3
                },
            ),
            _ => (
                if opcode.1 == 0 {
                    codes[i + 1] as usize
                } else {
                    i + 1
                },
                if opcode.2 == 0 {
                    codes[i + 2] as usize
                } else {
                    i + 2
                },
                0,
            ),
        };
        match opcode.0 {
            Opcode::Sum(step) => {
                codes[params.2] = codes[params.0] + codes[params.1];
                i += step;
            }
            Opcode::Product(step) => {
                codes[params.2] = codes[params.0] * codes[params.1];
                i += step;
            }
            Opcode::Input(step) => {
                if is_first {
                    codes[params.0] = *init;
                    is_first = false;
                } else {
                    codes[params.0] = rx.recv().unwrap();
                }
                i += step;
            }
            Opcode::Output(step) => {
                match tx.send(codes[params.0]) {
                    Ok(_) => {}
                    Err(_) => {
                        return codes[params.0];
                    }
                }
                i += step;
            }
            Opcode::Halt(_) => {
                break;
            }
            Opcode::JumpT(step) => {
                if codes[params.0] != 0 {
                    i = codes[params.1] as usize;
                } else {
                    i += step;
                }
            }
            Opcode::JumpF(step) => {
                if codes[params.0] == 0 {
                    i = codes[params.1] as usize;
                } else {
                    i += step;
                }
            }
            Opcode::Less(step) => {
                codes[params.2] = if codes[params.0] < codes[params.1] {
                    1
                } else {
                    0
                };
                i += step;
            }
            Opcode::Equal(step) => {
                codes[params.2] = if codes[params.0] == codes[params.1] {
                    1
                } else {
                    0
                };
                i += step;
            }
        }
    }
    0
}

fn parse_opcode(opcode: isize) -> (Opcode, usize, usize, usize) {
    let mut s_opcode = opcode.to_string();
    let length = s_opcode.len();
    s_opcode = format!("{}{}", "0".repeat(5 - length), opcode.to_string());
    (
        match s_opcode[3..].parse::<usize>().unwrap() {
            1 => Opcode::Sum(4),
            2 => Opcode::Product(4),
            3 => Opcode::Input(2),
            4 => Opcode::Output(2),
            5 => Opcode::JumpT(3),
            6 => Opcode::JumpF(3),
            7 => Opcode::Less(4),
            8 => Opcode::Equal(4),
            _ => Opcode::Halt(1),
        },
        s_opcode[2..3].parse::<usize>().unwrap(),
        s_opcode[1..2].parse::<usize>().unwrap(),
        s_opcode[0..1].parse::<usize>().unwrap(),
    )
}
