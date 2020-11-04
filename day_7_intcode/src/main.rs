//use std::io;
use std::env;
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
    Halt(usize),
}

fn main() {
    let mut args = env::args();
    args.next();
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
                if opcode.3 == 0 {
                    codes[i + 3] as usize
                } else {
                    i + 3
                },
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
                let arg = args.next().unwrap();
                //println!("{}", arg);
                /*let mut input = String::new();
                //println!("Input:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
*/
                let input: isize = match arg.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                codes[params.0] = input;
                i += step;
            }
            Opcode::Output(step) => {
                println!("{}", codes[params.0]);
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
