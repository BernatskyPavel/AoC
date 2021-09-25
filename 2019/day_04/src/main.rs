enum Mode {
    First,
    Second
}

fn main() {
    println!("part_one - {}", task(265275, 781584, Mode::First));
    println!("part_two - {}", task(265275, 781584, Mode::Second));
}

fn task(min: u32, max: u32, mode: Mode) -> u32 {
    let func = match mode {
        Mode::First => {
            is_valid
        }
        Mode::Second => {
            is_valid_2
        }
    };
    (min..=max).filter(|&i| func(&i.to_string())).count() as u32
}

fn is_valid(string: &String) -> bool {
    if string.len() != 6 {
        return false;
    }
    let mut prev: char = ('0' as u8).saturating_sub(1) as char;
    let (mut sorted, mut pair) = (true, false);
    for ch in string.chars() {
        if ch < prev {
            sorted = false;
            break;
        } else if ch == prev {
            pair = true;
        }
        prev = ch;
    }
    sorted & pair
}

fn is_valid_2(string: &String) -> bool {
    if string.len() != 6 {
        return false;
    }
    let mut prev: char = ('0' as u8).saturating_sub(1) as char;
    let (mut sorted, mut pair) = (true, false);
    let mut counter = 0;
    for ch in string.chars() {
        if ch < prev {
            sorted = false;
            break;
        } else if ch == prev {
            counter += 1;
        } else {
            if counter == 2{
                pair = true;
            }
            counter = 1;
        }
        prev = ch;
    }
    if counter == 2{
        pair = true;
    }

    sorted & pair
}
