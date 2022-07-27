fn main() {
    println!("Part one: {}", part_one("ckczppom"));
    println!("Part two: {}", part_two("ckczppom"));
}

fn part_one(input: &str) -> usize {
    let mut i: usize = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i).as_bytes());
        match format!("{:?}", hash).strip_prefix("00000") {
            Some(_) => {
                break;
            }
            None => {
                i += 1;
            }
        }
    }
    i
}

fn part_two(input: &str) -> usize {
    let mut i: usize = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i).as_bytes());
        match format!("{:?}", hash).strip_prefix("000000") {
            Some(_) => {
                break;
            }
            None => {
                i += 1;
            }
        }
    }
    i
}
