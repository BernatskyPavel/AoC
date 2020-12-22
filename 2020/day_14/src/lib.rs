use std::collections::BTreeMap;

#[derive(Clone)]
enum BitState {
    One,
    Zero,
    Unknown,
}

#[derive(Clone)]
pub struct Mask {
    size: u8,
    mask: Vec<BitState>,
}

impl Mask {
    fn empty() -> Mask {
        Mask {
            size: 36,
            mask: vec![BitState::Unknown; 36],
        }
    }

    fn new(mask: String) -> Mask {
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
            _ => {
                unimplemented!()
            }
        });
        Mask {
            size: mask.len() as u8,
            mask: vec_mask,
        }
    }
}

#[derive(Clone)]
pub struct App {
    memory: BTreeMap<usize, usize>,
    mask: Mask,
}
