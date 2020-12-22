use std::fs::File;
use std::io::{BufRead, BufReader};

use day_12::{App, Cmd, Operation};

fn main() {
    println!("Hello, world!");
    let operations = extract_program("input.txt");
    println!("{}", part_one(&operations));
    println!("{}", part_two(&operations));
}

const NORTH: &str = "N";
const EAST: &str = "E";
const WEST: &str = "W";
const SOUTH: &str = "S";
const LEFT: &str = "L";
const RIGHT: &str = "R";
const FORWARD: &str = "F";

fn part_one(operations: &Vec<Operation>) -> isize {  
    let mut app: App = App::new(&operations);
    app.process();
    app.calc_manh_dist((0,0))
}

fn part_two(operations: &Vec<Operation>) -> isize {
    let mut app: App = App::new(operations);
    app.switch_mode();
    app.process();
    app.calc_manh_dist((0,0))
}

fn extract_program(file: &str) -> Vec<Operation>{
    let file: File = File::open(file).unwrap();
    let mut operations: Vec<Operation> = Vec::new();
    let mut error_counter = 0;
    for line in BufReader::new(file).lines() {
        let mut string = line.unwrap();
        let arg = string.split_off(1);
        let cmd = match string.trim() {
            NORTH => Cmd::GoNorth,
            EAST => Cmd::GoEast,
            SOUTH => Cmd::GoSouth,
            WEST => Cmd::GoWest,
            LEFT => Cmd::TurnLeft,
            RIGHT => Cmd::TurnRight,
            FORWARD => Cmd::GoForward,
            _ => unreachable!(),
        };
        let option_arg = arg.parse::<isize>();
        if option_arg.is_err() {
            error_counter += 1;
            continue;
        }
        operations.push(Operation::new(&cmd, option_arg.unwrap()));
    }
    if error_counter != 0 {
        println!(
            "Errors: {}! Correct result is not guaranteed!",
            error_counter
        );
    } else {
        println!("File parsed with no errors!");
    };
    operations
}