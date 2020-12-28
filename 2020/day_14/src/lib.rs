use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
enum BitState {
    One,
    Zero,
    Unknown,
}

#[derive(Clone)]
enum AppVersion {
    V1,
    V2,
}

#[derive(Clone)]
pub struct Binaries {
    size: usize,
    bits: Vec<BitState>,
}

#[derive(Clone)]
pub enum Cmd {
    SetMask(String),
    Write(usize, usize),
}

impl Binaries {
    pub fn empty() -> Binaries {
        Binaries {
            size: 36,
            bits: vec![BitState::Unknown; 36],
        }
    }

    pub fn new(mask: String) -> Binaries {
        let mut vec_mask: Vec<BitState> = Vec::new();
        mask.chars().for_each(|ch| match ch {
            '0' => {
                vec_mask.push(BitState::Zero);
            }
            '1' => {
                vec_mask.push(BitState::One);
            }
            'X' => {
                vec_mask.push(BitState::Unknown);
            }
            _ => unimplemented!(),
        });
        Binaries {
            size: mask.len(),
            bits: vec_mask,
        }
    }

    fn process(&self, other: Binaries, version: &AppVersion) -> Option<Binaries> {
        if self.size != other.size {
            return None;
        };
        let mut i = 0;
        let mut result_bits: Vec<BitState> = Vec::new();
        result_bits.reserve_exact(self.size);
        loop {
            match version {
                AppVersion::V1 => {
                    match self.bits[i].clone() {
                        BitState::Unknown => {
                            result_bits.push(other.bits[i].clone());
                        }
                        _ => {
                            result_bits.push(self.bits[i].clone());
                        }
                    };
                }
                AppVersion::V2 => {
                    match self.bits[i].clone() {
                        BitState::Zero => {
                            result_bits.push(other.bits[i].clone());
                        }
                        _ => {
                            result_bits.push(self.bits[i].clone());
                        }
                    };
                }
            }
            if i == self.size - 1 {
                break;
            }
            i += 1;
        }
        Some(Binaries {
            size: self.size,
            bits: result_bits,
        })
    }

    pub fn to_usize(&self) -> Option<usize> {
        if self.bits.iter().any(|state| *state == BitState::Unknown) {
            return None;
        };
        let mut result = 0;
        self.bits
            .iter()
            .enumerate()
            .for_each(|state| match state.1 {
                BitState::One => {
                    result += 2_usize.pow((self.size - 1 - state.0) as u32);
                }
                _ => {}
            });
        Some(result)
    }

    pub fn to_usize_vec(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::new();
        let mut buf: Vec<Binaries> = vec![self.clone()];
        loop {
            let op_temp = buf.pop();
            if op_temp.is_none() {
                break;
            }
            let temp = op_temp.unwrap();
            let attempt = temp.to_usize();
            if attempt.is_some() {
                vec.push(attempt.unwrap());
                continue;
            }
            let mut i = 0;
            loop {
                if i == temp.size {
                    break;
                }
                if temp.bits[i] == BitState::Unknown {
                    let mut temp_zero = temp.clone();
                    temp_zero.bits[i] = BitState::Zero;
                    buf.push(temp_zero);
                    let mut temp_one = temp.clone();
                    temp_one.bits[i] = BitState::One;
                    buf.push(temp_one);
                    break;
                }
                i += 1;
            }
            if buf.is_empty() {
                break;
            }
        }
        vec
    }
}

#[derive(Clone)]
pub struct App {
    memory: BTreeMap<usize, usize>,
    mask: Binaries,
    ops: Vec<Cmd>,
    ops_readonly: Vec<Cmd>,
    version: AppVersion,
}

impl App {
    pub fn new(ops: &Vec<Cmd>) -> App {
        let mut app = App {
            memory: BTreeMap::new(),
            mask: Binaries::empty(),
            ops: Vec::new(),
            ops_readonly: ops.clone(),
            version: AppVersion::V1,
        };
        app.ops_readonly.reverse();
        app.ops = app.ops_readonly.clone();
        app
    }

    pub fn step(&mut self) {
        let op_option = self.ops.pop();
        if op_option.is_some() {
            let op = op_option.unwrap();
            match self.version {
                AppVersion::V1 => match op {
                    Cmd::SetMask(s_mask) => {
                        let mask = Binaries::new(s_mask);
                        self.mask = mask;
                    }
                    Cmd::Write(key, value) => {
                        let b_to = Binaries::new(format!("{:036b}", value));
                        let buf = self.mask.process(b_to, &self.version);
                        if buf.is_some() {
                            let buf = buf.unwrap().to_usize();
                            if buf.is_some() {
                                let entry = self.memory.entry(key).or_insert(buf.unwrap());
                                *entry = buf.unwrap();
                            } else {
                                println!("Bit processing error!");
                            }
                        } else {
                            println!("Mask processing error!");
                        }
                    }
                },
                AppVersion::V2 => match op {
                    Cmd::SetMask(s_mask) => {
                        let mask = Binaries::new(s_mask);
                        self.mask = mask;
                    }
                    Cmd::Write(key, value) => {
                        let b_address = Binaries::new(format!("{:036b}", key));
                        let buf = self.mask.process(b_address, &self.version);
                        if buf.is_some() {
                            let vec_adr = buf.unwrap().to_usize_vec();
                            vec_adr.iter().for_each(|&adr| {
                                let entry = self.memory.entry(adr).or_insert(value);
                                *entry = value;
                            });
                        } else {
                            println!("Mask processing error!");
                        }
                    }
                },
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
        self.memory = BTreeMap::new();
    }

    pub fn load_new_program(&mut self, program: &Vec<Cmd>) {
        self.ops_readonly = program.clone();
        self.ops_readonly.reverse();
        self.reload();
    }

    pub fn get_memory_sum(&self) -> usize {
        self.memory.values().sum()
    }

    pub fn switch_version(&mut self) {
        self.version = match self.version {
            AppVersion::V1 => AppVersion::V2,
            AppVersion::V2 => AppVersion::V1,
        };
    }
}
