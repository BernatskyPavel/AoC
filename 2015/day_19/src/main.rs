use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const SPLIT_SEQ: &str = " => ";

fn main() {
    println!("Part one: {}", part_one("input.txt"));
}

fn part_one(file_path: &str) -> usize {
    let (seq, rules) = extract_input(file_path);
    let mut seq_set = HashSet::<String>::new();

    for (seq_part, rule) in rules {
        for swap_seq in rule {
            let indices = seq
                .match_indices(seq_part.as_str())
                .map(|i| i.0)
                .collect::<Vec<_>>();
            for index in indices {
                seq_set.insert(format!(
                    "{}{}{}",
                    &seq[..index],
                    swap_seq,
                    &seq[index + seq_part.len()..]
                ));
            }
        }
    }

    seq_set.len()
}

fn extract_input(file_path: &str) -> (String, HashMap<String, Vec<String>>) {
    let file: File = File::open(file_path).unwrap();
    let mut rules = HashMap::<String, Vec<String>>::new();
    let mut seq = String::new();
    let mut is_seq = false;
    for line in BufReader::new(file).lines().flatten() {
        if line.is_empty() {
            is_seq = true;
            continue;
        }
        if !is_seq {
            let parts = line.split(SPLIT_SEQ).collect::<Vec<_>>();
            let (key, value) = (parts[0].to_string(), parts[1].to_string());
            let entry = rules.entry(key).or_insert(Vec::new());
            entry.push(value);
        } else {
            seq = line;
        }
    }
    (seq, rules)
}

#[test]
fn test_part_one() {
    assert_eq!(part_one("test.txt"), 4);
    assert_eq!(part_one("test2.txt"), 7);
}
