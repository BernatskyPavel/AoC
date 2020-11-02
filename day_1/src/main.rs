use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file: File = File::open( "file.txt").unwrap();
    let mut sum = 0;
    for line in BufReader::new(file).lines(){
        let mut number = line.unwrap().parse::<u32>().unwrap();
        loop {
            let temp =  number / 3;
            if temp == 0 {
                break;
            } else {
                number = temp.saturating_sub(2);
                sum += number;
            }
        }
        //sum += number / 3 - 2;
    }
    println!("{}", sum);
}
