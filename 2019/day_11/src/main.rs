use std::collections::BTreeMap;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};
use std::thread::Builder;

fn codes() -> Vec<&'static isize> {
    [
        3_isize,
        8,
        1005,
        8,
        299,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1002,
        8,
        1,
        29,
        1,
        1007,
        14,
        10,
        2,
        1106,
        8,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        58,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        80,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        102,
        1,
        8,
        103,
        1,
        5,
        6,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        101,
        0,
        8,
        128,
        1,
        106,
        18,
        10,
        1,
        7,
        20,
        10,
        1006,
        0,
        72,
        1006,
        0,
        31,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        164,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        186,
        1,
        1007,
        8,
        10,
        1006,
        0,
        98,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        101,
        0,
        8,
        216,
        2,
        102,
        8,
        10,
        1,
        1008,
        18,
        10,
        1,
        1108,
        8,
        10,
        1006,
        0,
        68,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1001,
        8,
        0,
        253,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        274,
        1,
        1105,
        7,
        10,
        101,
        1,
        9,
        9,
        1007,
        9,
        1060,
        10,
        1005,
        10,
        15,
        99,
        109,
        621,
        104,
        0,
        104,
        1,
        21102,
        936995738520,
        1,
        1,
        21102,
        316,
        1,
        0,
        1106,
        0,
        420,
        21101,
        0,
        936995824276,
        1,
        21102,
        1,
        327,
        0,
        1106,
        0,
        420,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21102,
        248129784923,
        1,
        1,
        21102,
        1,
        374,
        0,
        1105,
        1,
        420,
        21102,
        29015149735,
        1,
        1,
        21101,
        385,
        0,
        0,
        1106,
        0,
        420,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21101,
        983925826304,
        0,
        1,
        21101,
        0,
        408,
        0,
        1105,
        1,
        420,
        21102,
        825012036364,
        1,
        1,
        21101,
        0,
        419,
        0,
        1105,
        1,
        420,
        99,
        109,
        2,
        22101,
        0,
        -1,
        1,
        21101,
        0,
        40,
        2,
        21101,
        0,
        451,
        3,
        21102,
        441,
        1,
        0,
        1105,
        1,
        484,
        109,
        -2,
        2105,
        1,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        446,
        447,
        462,
        4,
        0,
        1001,
        446,
        1,
        446,
        108,
        4,
        446,
        10,
        1006,
        10,
        478,
        1101,
        0,
        0,
        446,
        109,
        -2,
        2105,
        1,
        0,
        0,
        109,
        4,
        2102,
        1,
        -1,
        483,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        501,
        21102,
        0,
        1,
        -3,
        21201,
        -3,
        0,
        1,
        22102,
        1,
        -2,
        2,
        21102,
        1,
        1,
        3,
        21101,
        520,
        0,
        0,
        1106,
        0,
        525,
        109,
        -4,
        2105,
        1,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        548,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        548,
        21201,
        -4,
        0,
        -4,
        1105,
        1,
        616,
        21201,
        -4,
        0,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21102,
        1,
        567,
        0,
        1105,
        1,
        525,
        21202,
        1,
        1,
        -4,
        21102,
        1,
        1,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        586,
        21102,
        0,
        1,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        608,
        21201,
        -1,
        0,
        1,
        21102,
        1,
        608,
        0,
        106,
        0,
        483,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ]
    .iter()
    .collect()
}

fn main() {
    part_one();
}

impl Direction {
    fn rotate(self, value: isize) -> Direction {
        match self {
            Direction::Up => match value {
                0 => Direction::Left,
                1 => Direction::Right,
                _ => unreachable!(),
            },
            Direction::Down => match value {
                0 => Direction::Right,
                1 => Direction::Left,
                _ => unreachable!(),
            },
            Direction::Right => match value {
                0 => Direction::Up,
                1 => Direction::Down,
                _ => unreachable!(),
            },
            Direction::Left => match value {
                0 => Direction::Down,
                1 => Direction::Up,
                _ => unreachable!(),
            },
        }
    }
    fn change_point(self, point: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (point.0, point.1 + 1),
            Direction::Down => (point.0, point.1 - 1),
            Direction::Right => (point.0 + 1, point.1),
            Direction::Left => (point.0 - 1, point.1),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn part_one() {
    let (tx, rx_1) = mpsc::channel::<isize>();
    let (tx_1, rx) = mpsc::channel::<(isize, isize)>();
    let (black, white) = (0, 1);
    let mut points: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    points.insert((0,0), white);
    let mut _current_point = (0, 0);
    let mut current_direction = Direction::Up;
    let temp = Arc::new(Mutex::new(points));
    {
        Builder::new()
            .name("intcode".into())
            .spawn(move || intcode(rx_1, tx_1, codes()))
            .unwrap();

        let temp = Arc::clone(&temp);
        Builder::new()
            .name("rover".into())
            .spawn(move || {
                let mut mutex = temp.lock().unwrap();

                loop {
                    let mut _pointer = mutex.entry(_current_point).or_insert(black);
                    tx.send(*_pointer).expect("Send error!");
                    let step = rx.recv().expect("Recieve error!");
                    if step == (-1, -1) {
                        break;
                    }
                    *_pointer = step.0;
                    current_direction = current_direction.rotate(step.1);
                    _current_point = current_direction.change_point(_current_point);
                }
            })
            .unwrap()
            .join()
            .expect("x");
    }
    
    let result = temp.lock().unwrap();
    let (mut left,mut right, mut up, mut down) = (0,0,0,0);
    result.iter().for_each(|x| {
        if x.0.0 < left {
            left = x.0.0;
        }
        if x.0.0 > right {
            right = x.0.0;
        }
        if x.0.1 < down {
            down = x.0.1;
        }
        if x.0.1 > up {
            up = x.0.1;
        }
    });
    let (diff_x, diff_y) = (left.abs() + right.abs(), up.abs() + down.abs());
    let mut logo: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    result.iter().for_each(|point| {
        let x = (point.0.0 + left.abs()) as usize;
        let y = (point.0.1 + down.abs()) as usize;
        logo.entry(y).or_insert(BTreeMap::new()).entry(x).or_insert(*point.1 as usize);
    });
    for y in 0..=diff_y {
        for x in 0..=diff_x {
            logo.entry(y as usize).or_insert(BTreeMap::new()).entry(x as usize).or_insert(1);
        }
    }
    let mut logo_v = logo.values().map(|x|{
        x.values().map(|val| val.to_string()).collect::<Vec<String>>().join("")
    }).collect::<Vec<String>>();
    logo_v.reverse();
    println!("{}", logo_v.join("\n"));
}
#[derive(Debug)]
enum Opcode {
    Sum(usize),
    Product(usize),
    Input(usize),
    Output(usize),
    JumpT(usize),
    JumpF(usize),
    Less(usize),
    Equal(usize),
    Offset(usize),
    Halt(usize),
}

fn intcode(rx: Receiver<isize>, tx: Sender<(isize, isize)>, code: Vec<&isize>) {
    let mut codes = code
        .iter()
        .enumerate()
        .map(|x| (x.0, **x.1 as isize))
        .collect::<BTreeMap<usize, isize>>();
    let mut is_paired = false;
    let mut output: (isize, isize) = (0, 0);
    let length = codes.len();
    let mut i = 0;
    let mut offset_base: isize = 0;
    loop {
        if i >= length {
            break;
        }
        let opcode = parse_opcode(*codes.get(&i).unwrap());

        let params = match opcode.0 {
            Opcode::Halt(_) => (0_isize, 0_isize, 0_isize),
            _ => (
                match opcode.1 {
                    0 => *codes.entry(i + 1 as usize).or_insert(0),
                    1 => i as isize + 1,
                    2 => offset_base + *codes.entry(i + 1 as usize).or_insert(0),
                    _ => unreachable!(),
                },
                match opcode.2 {
                    0 => *codes.entry(i + 2 as usize).or_insert(0),
                    1 => i as isize + 2,
                    2 => offset_base + *codes.entry(i + 2 as usize).or_insert(0),
                    _ => unreachable!(),
                },
                match opcode.3 {
                    0 => *codes.entry(i + 3 as usize).or_insert(0),
                    1 => i as isize + 3,
                    2 => offset_base + *codes.entry(i + 3 as usize).or_insert(0),
                    _ => unreachable!(),
                },
            ),
        };
        match opcode.0 {
            Opcode::Sum(step) => {
                *codes.entry(params.2 as usize).or_insert(0) =
                    *codes.entry(params.0 as usize).or_insert(0)
                        + *codes.entry(params.1 as usize).or_insert(0);
                i += step;
            }
            Opcode::Product(step) => {
                *codes.entry(params.2 as usize).or_insert(0) =
                    *codes.entry(params.0 as usize).or_insert(0)
                        * *codes.entry(params.1 as usize).or_insert(0);
                i += step;
            }
            Opcode::Input(step) => {
                *codes.entry(params.0 as usize).or_insert(0) = rx.recv().expect("Receive error!");
                i += step;
            }
            Opcode::Output(step) => {
                if !is_paired {
                    output.0 = *codes.entry(params.0 as usize).or_insert(0);
                } else {
                    output.1 = *codes.entry(params.0 as usize).or_insert(0);
                    tx.send(output).expect("Sending error!");
                }
                is_paired = !is_paired;
                i += step;
            }
            Opcode::Halt(_) => {
                tx.send((-1,-1)).expect("Sending error!");
                break;
            }
            Opcode::JumpT(step) => {
                if *codes.entry(params.0 as usize).or_insert(0) != 0 {
                    i = *codes.entry(params.1 as usize).or_insert(0) as usize;
                } else {
                    i += step;
                }
            }
            Opcode::JumpF(step) => {
                if *codes.entry(params.0 as usize).or_insert(0) == 0 {
                    i = *codes.entry(params.1 as usize).or_insert(0) as usize;
                } else {
                    i += step;
                }
            }
            Opcode::Less(step) => {
                *codes.entry(params.2 as usize).or_insert(0) =
                    if *codes.entry(params.0 as usize).or_insert(0)
                        < *codes.entry(params.1 as usize).or_insert(0)
                    {
                        1
                    } else {
                        0
                    };
                i += step;
            }
            Opcode::Equal(step) => {
                *codes.entry(params.2 as usize).or_insert(0) =
                    if *codes.entry(params.0 as usize).or_insert(0)
                        == *codes.entry(params.1 as usize).or_insert(0)
                    {
                        1
                    } else {
                        0
                    };
                i += step;
            }
            Opcode::Offset(step) => {
                offset_base += *codes.entry(params.0 as usize).or_insert(0);
                i += step;
            }
        }
    }
}

fn parse_opcode(opcode: isize) -> (Opcode, isize, isize, isize) {
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
            9 => Opcode::Offset(2),
            _ => Opcode::Halt(1),
        },
        s_opcode[2..3].parse::<isize>().unwrap(),
        s_opcode[1..2].parse::<isize>().unwrap(),
        s_opcode[0..1].parse::<isize>().unwrap(),
    )
}
