#![allow(dead_code)]
use std::fs::File;
use std::io::Read;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;
const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    part_two();
}

fn part_one() {
    let (length, height) = (25, 6);
    let square = length * height;
    let image: &mut Vec<u8> = &mut Vec::new();
    {
        let mut file: File = File::open("image.txt").unwrap();
        file.read_to_end(image).expect("Read file failed!");
    }
    let mut layers: Vec<Vec<&u8>> = Vec::new();
    (0..image.len()).step_by(square).for_each(|i| {
        layers.push(
            image
                .get(i.saturating_sub(square)..i)
                .unwrap()
                .iter()
                .collect::<Vec<&u8>>(),
        )
    });

    layers.sort_by(|a, b| {
        a.iter()
            .filter(|&&&x| x == ('0' as u8))
            .count()
            .partial_cmp(&b.iter().filter(|&&&x| x == ('0' as u8)).count())
            .unwrap()
    });

    let min_layer = layers.iter().nth(1).unwrap();

    println!(
        "{:?}",
        min_layer.iter().filter(|x| { ***x == ('1' as u8) }).count()
            * min_layer.iter().filter(|&&&x| { x == ('2' as u8) }).count()
    );
}

fn part_two() {
    let image: &mut Vec<u8> = &mut Vec::new();
    {
        let mut file: File = File::open("image.txt").unwrap();
        file.read_to_end(image).expect("Read file failed!");
    }

    let mut message: [u8; SIZE] = [
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
        50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
    ];
    (0..=image.len()).step_by(SIZE).for_each(|i| {
        image
            .get(i.saturating_sub(SIZE)..i)
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|pixel| {
                match message[pixel.0] {
                    50 => {
                        message[pixel.0] = *(pixel.1);
                    },
                    _ => {},
                }
            });
    });

    (0..message.len()).for_each(|i|{
        if i % WIDTH == 0 {
            print!("\n");
        }
        print!("{}", if message[i] == 49 {"X"} else {" "});
    });
}
