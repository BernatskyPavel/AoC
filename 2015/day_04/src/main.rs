fn main() {
    println!("Hello, world!");
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let input = "ckczppom";
    let mut i: usize = 0;
    loop {
        let hash = md5::compute(format!("{}{}",input,i).as_bytes());
        match format!("{:?}", hash).strip_prefix("00000") {
            Some(_) => {
                break;
            },
            None => {
                i+=1;
            },
        }
    };
    i
}

fn part_two() -> usize {
    let input = "ckczppom";
    let mut i: usize = 0;
    loop {
        let hash = md5::compute(format!("{}{}",input,i).as_bytes());
        match format!("{:?}", hash).strip_prefix("000000") {
            Some(_) => {
                break;
            },
            None => {
                i+=1;
            },
        }
    };
    i
}
