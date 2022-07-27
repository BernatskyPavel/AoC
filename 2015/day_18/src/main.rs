use std::fs::File;
use std::io::{BufRead, BufReader};

const ON: char = '#';
const OFF: char = '.';
const ADJUSENT: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

struct Light {
    pub is_on: bool,
    is_for_change: bool,
    is_const: bool,
}

impl Light {
    fn new(is_on: bool) -> Self {
        Light {
            is_on,
            is_for_change: false,
            is_const: false,
        }
    }

    fn change(&mut self) {
        if !self.is_const && self.is_for_change {
            self.is_on = !self.is_on;
        }
        self.is_for_change = false;
    }

    fn set_on_forever(&mut self) {
        self.is_on = true;
        self.is_const = true;
    }
}

fn main() {
    println!("Part one: {}", part_one("input.txt", 100, false));
    println!("Part two: {}", part_two("input.txt", 100));
}

fn part_one(file_path: &str, cycle_len: usize, is_modified: bool) -> usize {
    let mut map = extract_map_from_file(file_path);
    let y_len = map.len();

    if is_modified {
        map[0][0].set_on_forever();
        map[y_len - 1][0].set_on_forever();
        map[0][y_len - 1].set_on_forever();
        map[y_len - 1][y_len - 1].set_on_forever();
    }

    for _ in 0..cycle_len {
        for y in 0..y_len {
            let x_len = map[y].len();
            for x in 0..x_len {
                let mut counter = 0;
                for neighbour in ADJUSENT {
                    let neigh_x = x as isize + neighbour.0;
                    let neigh_y = y as isize + neighbour.1;
                    if neigh_x < 0
                        || neigh_y < 0
                        || neigh_x >= x_len as isize
                        || neigh_y >= y_len as isize
                    {
                        continue;
                    }
                    if map[neigh_y as usize][neigh_x as usize].is_on {
                        counter += 1;
                    }
                }
                match (map[y][x].is_on, counter) {
                    (true, 2 | 3) => {}
                    (false, 3) | (true, _) => {
                        map[y][x].is_for_change = true;
                    }
                    _ => {}
                };
            }
        }
        map.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|light| light.change()));
    }
    let mut result = 0;
    for y in 0..y_len {
        let x_len = map[y].len();
        for x in 0..x_len {
            if map[y][x].is_on {
                result += 1;
            }
        }
    }
    result
}

fn part_two(file_path: &str, cycle_len: usize) -> usize {
    part_one(file_path, cycle_len, true)
}

fn extract_map_from_file(file_path: &str) -> Vec<Vec<Light>> {
    let mut result: Vec<Vec<Light>> = Vec::new();
    let file: File = File::open(file_path).unwrap();
    for line in BufReader::new(file).lines() {
        let mut temp: Vec<Light> = Vec::new();
        for ch in line.unwrap().chars() {
            temp.push(Light::new(match ch {
                OFF => false,
                ON => true,
                _ => unreachable!(),
            }));
        }
        result.push(temp);
    }
    result
}

#[test]
fn test_part_one() {
    assert_eq!(part_one("test.txt", 4, false), 4);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two("test2.txt", 4), 14);
    assert_eq!(part_two("test2.txt", 5), 17);
}
