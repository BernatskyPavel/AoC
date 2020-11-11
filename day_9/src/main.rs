use std::io;
use std::collections::BTreeMap;
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

fn main() {
    let mut codes = [
        1102_isize, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000,
        109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008,
        1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104,
        0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 36, 1015, 1102, 1, 387, 1028, 1101, 24, 0, 1016,
        1101, 0, 23, 1008, 1102, 1, 35, 1012, 1102, 1, 554, 1023, 1101, 29, 0, 1003, 1101, 27, 0,
        1011, 1101, 25, 0, 1000, 1101, 0, 38, 1018, 1102, 20, 1, 1019, 1102, 28, 1, 1005, 1102, 1,
        619, 1026, 1102, 1, 22, 1004, 1101, 0, 0, 1020, 1101, 0, 31, 1009, 1102, 1, 783, 1024,
        1102, 1, 33, 1001, 1102, 616, 1, 1027, 1102, 1, 21, 1006, 1101, 32, 0, 1013, 1102, 39, 1,
        1014, 1102, 1, 378, 1029, 1101, 774, 0, 1025, 1102, 1, 1, 1021, 1102, 30, 1, 1007, 1102,
        37, 1, 1002, 1102, 1, 26, 1017, 1101, 0, 557, 1022, 1102, 1, 34, 1010, 109, 13, 2101, 0,
        -5, 63, 1008, 63, 23, 63, 1005, 63, 203, 4, 187, 1105, 1, 207, 1001, 64, 1, 64, 1002, 64,
        2, 64, 109, -14, 2107, 28, 4, 63, 1005, 63, 225, 4, 213, 1106, 0, 229, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, 10, 1207, -3, 20, 63, 1005, 63, 245, 1106, 0, 251, 4, 235, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, 8, 1205, 3, 263, 1105, 1, 269, 4, 257, 1001, 64, 1, 64, 1002,
        64, 2, 64, 109, -9, 1207, -7, 34, 63, 1005, 63, 287, 4, 275, 1105, 1, 291, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -4, 2102, 1, -3, 63, 1008, 63, 32, 63, 1005, 63, 311, 1105, 1, 317,
        4, 297, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 21, 21101, 40, 0, -6, 1008, 1019, 43, 63,
        1005, 63, 337, 1106, 0, 343, 4, 323, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -26, 1202, 7,
        1, 63, 1008, 63, 21, 63, 1005, 63, 365, 4, 349, 1106, 0, 369, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, 26, 2106, 0, 3, 4, 375, 1001, 64, 1, 64, 1105, 1, 387, 1002, 64, 2, 64, 109, -9,
        21108, 41, 40, 3, 1005, 1019, 407, 1001, 64, 1, 64, 1106, 0, 409, 4, 393, 1002, 64, 2, 64,
        109, 13, 1205, -8, 423, 4, 415, 1106, 0, 427, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -19,
        21107, 42, 41, 5, 1005, 1015, 447, 1001, 64, 1, 64, 1106, 0, 449, 4, 433, 1002, 64, 2, 64,
        109, -3, 2102, 1, -5, 63, 1008, 63, 37, 63, 1005, 63, 471, 4, 455, 1105, 1, 475, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, -2, 1201, 0, 0, 63, 1008, 63, 28, 63, 1005, 63, 497, 4, 481,
        1105, 1, 501, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 8, 2107, 29, -8, 63, 1005, 63, 521,
        1001, 64, 1, 64, 1106, 0, 523, 4, 507, 1002, 64, 2, 64, 109, -3, 1208, -3, 30, 63, 1005,
        63, 541, 4, 529, 1106, 0, 545, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 4, 2105, 1, 9, 1105,
        1, 563, 4, 551, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 9, 1206, -3, 581, 4, 569, 1001, 64,
        1, 64, 1106, 0, 581, 1002, 64, 2, 64, 109, -8, 1201, -9, 0, 63, 1008, 63, 23, 63, 1005, 63,
        605, 1001, 64, 1, 64, 1106, 0, 607, 4, 587, 1002, 64, 2, 64, 109, 21, 2106, 0, -9, 1106, 0,
        625, 4, 613, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -35, 2108, 31, 8, 63, 1005, 63, 647, 4,
        631, 1001, 64, 1, 64, 1105, 1, 647, 1002, 64, 2, 64, 109, 2, 1202, 0, 1, 63, 1008, 63, 30,
        63, 1005, 63, 667, 1105, 1, 673, 4, 653, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 21108,
        43, 43, -4, 1005, 1016, 691, 4, 679, 1106, 0, 695, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
        -14, 1208, -1, 30, 63, 1005, 63, 711, 1106, 0, 717, 4, 701, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, 6, 21101, 44, 0, -1, 1008, 1011, 44, 63, 1005, 63, 739, 4, 723, 1105, 1, 743,
        1001, 64, 1, 64, 1002, 64, 2, 64, 109, -15, 2108, 30, 8, 63, 1005, 63, 759, 1106, 0, 765,
        4, 749, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 27, 2105, 1, 0, 4, 771, 1001, 64, 1, 64,
        1105, 1, 783, 1002, 64, 2, 64, 109, -9, 1206, 6, 795, 1105, 1, 801, 4, 789, 1001, 64, 1,
        64, 1002, 64, 2, 64, 109, 4, 21102, 45, 1, -7, 1008, 1012, 45, 63, 1005, 63, 823, 4, 807,
        1105, 1, 827, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 21102, 46, 1, 5, 1008, 1010, 43,
        63, 1005, 63, 851, 1001, 64, 1, 64, 1105, 1, 853, 4, 833, 1002, 64, 2, 64, 109, -1, 2101,
        0, 1, 63, 1008, 63, 25, 63, 1005, 63, 873, 1105, 1, 879, 4, 859, 1001, 64, 1, 64, 1002, 64,
        2, 64, 109, 9, 21107, 47, 48, -3, 1005, 1010, 897, 4, 885, 1105, 1, 901, 1001, 64, 1, 64,
        4, 64, 99, 21101, 0, 27, 1, 21101, 915, 0, 0, 1106, 0, 922, 21201, 1, 57526, 1, 204, 1, 99,
        109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 942, 0, 0, 1106, 0, 922,
        21201, 1, 0, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0, 922, 22201, 1, -1, -2, 1105,
        1, 968, 21202, -2, 1, -2, 109, -3, 2106, 0, 0,
    ]
    .iter()
    .enumerate()
    .map(|x| (x.0, *x.1 as isize))
    .collect::<BTreeMap<usize, isize>>();
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
        // println!("{:?}-{:?}", opcode, params);
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
                let mut input = String::new();
                println!("Input:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input: isize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                *codes.entry(params.0 as usize).or_insert(0) = input;
                i += step;
            }
            Opcode::Output(step) => {
                println!("Code: {}", *codes.entry(params.0 as usize).or_insert(0));
                i += step;
            }
            Opcode::Halt(_) => {
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
