use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{:?}", part_two("input.txt"));
}

const FLOOR: char = '.';
const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
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

#[derive(Clone, Copy, Debug)]
struct Place {
    is_seat: bool,
    is_occupied: bool,
    is_for_change: bool,
}

impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        self.is_seat == other.is_seat && self.is_occupied == other.is_occupied
    }
}

impl Place {
    fn new(is_seat: bool, is_occupied: bool, is_for_change: bool) -> Place {
        Place {
            is_seat,
            is_occupied,
            is_for_change,
        }
    }
    fn change(&mut self) {
        if self.is_for_change && self.is_seat {
            self.is_for_change = false;
            self.is_occupied = !self.is_occupied;
        }
    }
}

fn part_one(file_path: &str) -> usize {
    let mut seats = extract_map_from_file(file_path);
    loop {
        for row in seats.clone().iter_mut().enumerate() {
            for place in row.1.iter_mut().enumerate() {
                if place.1.is_seat {
                    if !place.1.is_occupied {
                        let mut is_adjusent_free = true;
                        for (x, y) in ADJUSENT.iter() {
                            if row.0 as isize + *y < 0 || place.0 as isize + *x < 0 {
                                is_adjusent_free &= true;
                            } else {
                                let adj_row = seats.get((row.0 as isize + *y) as usize);
                                if adj_row.is_none() {
                                    is_adjusent_free &= true;
                                } else {
                                    let adj =
                                        adj_row.unwrap().get((place.0 as isize + *x) as usize);
                                    if adj.is_none() {
                                        is_adjusent_free &= true;
                                    } else {
                                        is_adjusent_free &= !adj.unwrap().is_occupied;
                                    }
                                }
                            }
                        }
                        if is_adjusent_free {
                            seats[row.0][place.0].is_for_change = true;
                        }
                    } else {
                        let mut counter = 0;
                        for (x, y) in ADJUSENT.iter() {
                            if row.0 as isize + *y < 0 || place.0 as isize + *x < 0 {
                                counter += 0;
                            } else {
                                let adj_row = seats.get((row.0 as isize + *y) as usize);
                                if adj_row.is_none() {
                                    counter += 0;
                                } else {
                                    let adj =
                                        adj_row.unwrap().get((place.0 as isize + *x) as usize);
                                    if adj.is_none() {
                                        counter += 0;
                                    } else {
                                        counter += if adj.unwrap().is_occupied { 1 } else { 0 };
                                    }
                                }
                            }
                        }
                        if counter > 3 {
                            seats[row.0][place.0].is_for_change = true;
                        }
                    }
                }
            }
        }
        if seats
            .iter()
            .enumerate()
            .all(|row| row.1.iter().all(|seat| !seat.is_for_change))
        {
            break;
        }
        for row in seats.iter_mut() {
            for place in row.iter_mut() {
                place.change();
            }
        }
    }
    let mut result = 0;
    seats.iter().enumerate().for_each(|row| {
        row.1.iter().enumerate().for_each(|place| {
            if place.1.is_seat && place.1.is_occupied {
                result += 1;
            }
        });
    });
    result
}

fn part_two(file_path: &str) -> usize {
    let mut seats = extract_map_from_file(file_path);
    loop {
        for row in seats.clone().iter_mut().enumerate() {
            for place in row.1.iter_mut().enumerate() {
                if place.1.is_seat {
                    if !place.1.is_occupied {
                        let mut is_adjusent_free = true;
                        for (x_0, y_0) in ADJUSENT.iter() {
                            let mut is_end = false;
                            let mut adj_inc = 0;
                            loop {
                                let (x, y) = (*x_0 * (1 + adj_inc), *y_0 * (1 + adj_inc));
                                if row.0 as isize + y < 0 || place.0 as isize + x < 0 {
                                    is_adjusent_free &= true;
                                    is_end = true;
                                } else {
                                    let adj_row = seats.get((row.0 as isize + y) as usize);
                                    if adj_row.is_none() {
                                        is_adjusent_free &= true;
                                        is_end = true;
                                    } else {
                                        let adj =
                                            adj_row.unwrap().get((place.0 as isize + x) as usize);
                                        if adj.is_none() {
                                            is_adjusent_free &= true;
                                            is_end = true;
                                        } else {
                                            let temp = adj.unwrap();
                                            if temp.is_seat {
                                                is_adjusent_free &= !temp.is_occupied;
                                                is_end = true;
                                            } else {
                                                is_adjusent_free &= true;
                                            }
                                        }
                                    }
                                }
                                if is_end {
                                    break;
                                }
                                adj_inc += 1;
                            }
                        }
                        if is_adjusent_free {
                            seats[row.0][place.0].is_for_change = true;
                        }
                    } else {
                        let mut counter = 0;
                        for (x_0, y_0) in ADJUSENT.iter() {
                            let mut is_end = false;
                            let mut adj_inc = 0;
                            loop {
                                let (x, y) = (*x_0 * (1 + adj_inc), *y_0 * (1 + adj_inc));
                                if row.0 as isize + y < 0 || place.0 as isize + x < 0 {
                                    counter += 0;
                                    is_end = true;
                                } else {
                                    let adj_row = seats.get((row.0 as isize + y) as usize);
                                    if adj_row.is_none() {
                                        counter += 0;
                                        is_end = true;
                                    } else {
                                        let adj =
                                            adj_row.unwrap().get((place.0 as isize + x) as usize);
                                        if adj.is_none() {
                                            counter += 0;
                                            is_end = true;
                                        } else {
                                            let temp = adj.unwrap();
                                            if temp.is_seat {
                                                counter += if adj.unwrap().is_occupied { 1 } else { 0 };
                                                is_end = true;
                                            }
                                        }
                                    }
                                }
                                if is_end {
                                    break;
                                }
                                adj_inc += 1;
                            }
                        }
                        if counter > 4 {
                            seats[row.0][place.0].is_for_change = true;
                        }
                    }
                }
            }
        }
        if seats
            .iter()
            .enumerate()
            .all(|row| row.1.iter().all(|seat| !seat.is_for_change))
        {
            break;
        }
        for row in seats.iter_mut() {
            for place in row.iter_mut() {
                place.change();
            }
        }
    }
    let mut result = 0;
    seats.iter().enumerate().for_each(|row| {
        row.1.iter().enumerate().for_each(|place| {
            if place.1.is_seat && place.1.is_occupied {
                result += 1;
            }
        });
    });
    result
}

fn extract_map_from_file(file_path: &str) -> Vec<Vec<Place>> {
    let mut result: Vec<Vec<Place>> = Vec::new();
    let file: File = File::open(file_path).unwrap();
    for line in BufReader::new(file).lines() {
        let mut temp: Vec<Place> = Vec::new();
        for ch in line.unwrap().chars() {
            let mut args = (false, false);
            match ch {
                EMPTY_SEAT => {
                    args.0 = true;
                }
                FLOOR => {}
                OCCUPIED_SEAT => {
                    args.0 = true;
                    args.1 = true;
                }
                _ => unreachable!(),
            }
            temp.push(Place::new(args.0, args.1, false));
        }
        result.push(temp);
    }
    result
}
