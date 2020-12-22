#[derive(Clone, Copy)]
pub enum Cmd {
    GoNorth,
    GoSouth,
    GoEast,
    GoWest,
    TurnLeft,
    TurnRight,
    GoForward,
}

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Waypoint,
}

#[derive(Clone, Copy)]
pub enum Direction {
    West,
    East,
    South,
    North,
}

impl Direction {
    pub fn rotate(&self, angle: isize) -> Direction {
        match angle.signum() {
            -1 => {
                let mut temp = *self;
                (0..angle.abs().div_euclid(90)).for_each(|_| {
                    temp = temp.rotate_counter_clockwise();
                });
                temp
            }
            0 => *self,
            1 => {
                let mut temp = *self;
                (0..angle.abs().div_euclid(90)).for_each(|_| {
                    temp = temp.rotate_clockwise();
                });
                temp
            }
            _ => unreachable!(),
        }
    }

    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::West => Direction::North,
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::West => Direction::South,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Operation {
    cmd: Cmd,
    arg: isize,
}

impl Operation {
    pub fn new(cmd: &Cmd, arg: isize) -> Operation {
        Operation {
            cmd: cmd.clone(),
            arg,
        }
    }

    pub fn get_cmd(&self) -> Cmd {
        self.cmd.clone()
    }

    pub fn get_arg(&self) -> isize {
        self.arg
    }
}

pub struct App {
    ops_readonly: Vec<Operation>,
    ops: Vec<Operation>,
    mode: Mode,
    current_direction: Direction,
    current_position: (isize, isize),
    waypoint_position: (isize, isize),
}

impl App {
    pub fn new(ops: &Vec<Operation>) -> App {
        let mut app = App {
            ops_readonly: ops.clone(),
            ops: ops.clone(),
            mode: Mode::Normal,
            current_direction: Direction::East,
            current_position: (0, 0),
            waypoint_position: (10, 1),
        };
        app.ops.reverse();
        app
    }

    pub fn step(&mut self) {
        let op_option = self.ops.pop();
        if op_option.is_some() {
            match self.mode {
                Mode::Normal => {
                    let op = op_option.unwrap();
                    match op.get_cmd() {
                        Cmd::GoNorth => {
                            self.current_position.1 += op.get_arg();
                        }
                        Cmd::GoSouth => {
                            self.current_position.1 -= op.get_arg();
                        }
                        Cmd::GoEast => {
                            self.current_position.0 += op.get_arg();
                        }
                        Cmd::GoWest => {
                            self.current_position.0 -= op.get_arg();
                        }
                        Cmd::TurnLeft => {
                            self.current_direction = self.current_direction.rotate(-op.get_arg());
                        }
                        Cmd::TurnRight => {
                            self.current_direction = self.current_direction.rotate(op.get_arg());
                        }
                        Cmd::GoForward => match self.current_direction {
                            Direction::North => {
                                self.current_position.1 += op.get_arg();
                            }
                            Direction::South => {
                                self.current_position.1 -= op.get_arg();
                            }
                            Direction::East => {
                                self.current_position.0 += op.get_arg();
                            }
                            Direction::West => {
                                self.current_position.0 -= op.get_arg();
                            }
                        },
                    }
                }
                Mode::Waypoint => {
                    let op = op_option.unwrap();
                    match op.get_cmd() {
                        Cmd::GoNorth => {
                            self.waypoint_position.1 += op.get_arg();
                        }
                        Cmd::GoSouth => {
                            self.waypoint_position.1 -= op.get_arg();
                        }
                        Cmd::GoEast => {
                            self.waypoint_position.0 += op.get_arg();
                        }
                        Cmd::GoWest => {
                            self.waypoint_position.0 -= op.get_arg();
                        }
                        Cmd::TurnLeft => {
                            self.waypoint_position = self.rotate_waypoint(-op.get_arg());
                        }
                        Cmd::TurnRight => {
                            self.waypoint_position = self.rotate_waypoint(op.get_arg());
                        }
                        Cmd::GoForward => {
                            self.current_position.0 += op.get_arg() * self.waypoint_position.0;
                            self.current_position.1 += op.get_arg() * self.waypoint_position.1;
                        }
                    }
                }
            }
        }
    }

    pub fn process(&mut self) {
        loop {
            self.step();
            if self.ops.len() == 0 {
                break;
            }
        }
    }

    pub fn reload(&mut self) {
        self.ops = self.ops_readonly.clone();
        self.ops.reverse();
        self.current_position = (0, 0);
        self.waypoint_position = (10, 1);
    }

    pub fn load_new_program(&mut self, program: &Vec<Operation>) {
        self.ops_readonly = program.clone();
        self.reload();
    }

    pub fn print_position(&self) {
        println!(
            "Current position: {} {} and {} {}",
            self.current_position.0.abs(),
            if self.current_position.0 < 0 {
                "West"
            } else {
                "East"
            },
            self.current_position.1.abs(),
            if self.current_position.1 < 0 {
                "South"
            } else {
                "North"
            }
        );
    }

    pub fn switch_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Normal => Mode::Waypoint,
            Mode::Waypoint => Mode::Normal,
        };
    }

    pub fn calc_manh_dist(&self, other: (isize, isize)) -> isize {
        (self.current_position.0 - other.0).abs() + (self.current_position.1 - other.1).abs()
    }

    fn rotate_waypoint(&self, angle: isize) -> (isize, isize) {
        let temp = vec![
            self.waypoint_position.0,
            self.waypoint_position.1,
            -self.waypoint_position.0,
            -self.waypoint_position.1,
        ];
        let mut result_position = (0, 1);
        (0..angle.abs().div_euclid(90)).for_each(|_| {
            result_position.0 = (4 + result_position.0 + angle.signum()) % 4;
            result_position.1 = (result_position.0 + 1) % 4;
        });
        (
            temp[result_position.0 as usize],
            temp[result_position.1 as usize],
        )
    }
}
