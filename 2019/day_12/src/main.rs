use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part_two();
    let duration = start.elapsed();
    println!("{:?}", duration);
}
#[derive(Debug, Clone, Copy)]
struct Moon {
    position: Position,
    velocity: Velocity,
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}
#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Moon {
        Moon {
            position: Position { x, y, z },
            velocity: Velocity { x: 0, y: 0, z: 0 },
        }
    }
    fn equal(self, other: Moon) -> bool {
        self.position.x == other.position.x
            && self.position.y == other.position.y
            && self.position.z == other.position.z
            && self.velocity.x == other.velocity.x
            && self.velocity.y == other.velocity.y
            && self.velocity.z == other.velocity.z
    }
    fn add_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

fn part_one() {
    let combinations = [0_usize, 1, 2, 3].iter().combinations(2);
    let combinations = combinations
        .map(|combination| (**combination.get(0).unwrap(), **combination.get(1).unwrap()))
        .collect::<Vec<(usize, usize)>>();
    let mut moons: [Moon; 4] = [
        Moon::new(8, 0, 8),
        Moon::new(0, -5, -10),
        Moon::new(16, 10, -5),
        Moon::new(19, -10, -7),
    ];
    let iterations = 1000;

    (0..iterations).for_each(|_| {
        combinations.iter().for_each(|&comb| {
            match moons[comb.0]
                .position
                .x
                .partial_cmp(&moons[comb.1].position.x)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.x += 1;
                    moons[comb.1].velocity.x -= 1;
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.x -= 1;
                    moons[comb.1].velocity.x += 1;
                }
            };
            match moons[comb.0]
                .position
                .y
                .partial_cmp(&moons[comb.1].position.y)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.y += 1;
                    moons[comb.1].velocity.y -= 1;
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.y -= 1;
                    moons[comb.1].velocity.y += 1;
                }
            };
            match moons[comb.0]
                .position
                .z
                .partial_cmp(&moons[comb.1].position.z)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.z += 1;
                    moons[comb.1].velocity.z -= 1;
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.z -= 1;
                    moons[comb.1].velocity.z += 1;
                }
            };
        });
        moons.iter_mut().for_each(|moon| {
            moon.position.x += moon.velocity.x;
            moon.position.y += moon.velocity.y;
            moon.position.z += moon.velocity.z;
        });
        //println!("{:?}", moons);
    });
    println!(
        "{:?}",
        moons
            .iter()
            .map(|moon| {
                (moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs())
                    * (moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs())
            })
            .collect::<Vec<isize>>()
            .iter()
            .sum::<isize>()
    );

    //println!("{:?}", moons);
}

fn part_two() {
    let combinations = [0_usize, 1, 2, 3].iter().combinations(2);
    let combinations = combinations
        .map(|combination| (**combination.get(0).unwrap(), **combination.get(1).unwrap()))
        .collect::<Vec<(usize, usize)>>();
    /*let mut moons: [Moon; 4] = [
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];*/
    let mut moons: [Moon; 4] = [
        Moon::new(-8, -10, 0),
        Moon::new(5, 5, 10),
        Moon::new(2, -7, 3),
        Moon::new(9, -8, -3),
    ];
    let init_state = moons.clone();
    let mut iteration: usize = 0;
    let mut moons_return_iterations = [0, 0, 0, 0];
    let mut moons_counter = 0;
    let mut step = 1;
    loop {
        combinations.iter().for_each(|&comb| {
            match moons[comb.0]
                .position
                .x
                .partial_cmp(&moons[comb.1].position.x)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.x += 1;
                    moons[comb.1].velocity.x -= 1;
                },
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.x -= 1;
                    moons[comb.1].velocity.x += 1;
                },
                std::cmp::Ordering::Equal => {},
            };
            match moons[comb.0]
                .position
                .y
                .partial_cmp(&moons[comb.1].position.y)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.y += 1;
                    moons[comb.1].velocity.y -= 1;
                },
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.y -= 1;
                    moons[comb.1].velocity.y += 1;
                },
                std::cmp::Ordering::Equal => {},
            };
            match moons[comb.0]
                .position
                .z
                .partial_cmp(&moons[comb.1].position.z)
                .unwrap()
            {
                std::cmp::Ordering::Less => {
                    moons[comb.0].velocity.z += 1;
                    moons[comb.1].velocity.z -= 1;
                },
                std::cmp::Ordering::Greater => {
                    moons[comb.0].velocity.z -= 1;
                    moons[comb.1].velocity.z += 1;
                },
                std::cmp::Ordering::Equal => {},
            };
        });
        moons.iter_mut().for_each(|moon| moon.add_velocity());
        iteration += 1;
        if iteration % step == 0 {
            moons.iter().enumerate().for_each(|moon| {
                if moons_return_iterations[moon.0] == 0 {
                    if moon.1.equal(init_state[moon.0]) {
                        moons_return_iterations[moon.0] = iteration;
                        step = lcm(step, iteration);
                        moons_counter += 1;
                    }
                }
            });
        }
        if moons_counter == 1 {
            break;
        }
        /*if moons_return_iterations.iter().all(|pos| *pos != 0) {
            break;
        }*/
        //println!("{:?}", moons);
    }
    println!("{:?}", moons_return_iterations);

    //println!("{:?}", moons);
}

fn gcd(a: usize, b: usize) -> usize {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
