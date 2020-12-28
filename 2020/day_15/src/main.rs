use std::collections::BTreeMap;

fn main() {
    let numbers = vec![12, 1, 16, 3, 11, 0];
    println!("{}", part_one(&numbers, 2020));
    println!("{}", part_one(&numbers, 30000000));
}

fn part_one(numbers: &Vec<usize>, search_index: usize) -> usize {
    if search_index < 1 {
        return 0;
    }
    if search_index >= 1 && search_index <= numbers.len() {
        return numbers[search_index - 1];
    }
    let mut i = numbers.len();
    let mut mem: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    numbers.iter().enumerate().for_each(|number| {
        mem.entry(*number.1)
            .and_modify(|x| {
                x.0 = x.1;
                x.1 = number.0 + 1;
            })
            .or_insert((0, number.0 + 1));
    });
    let mut current = mem.get(numbers.last().unwrap()).unwrap().0;
    let result = loop {
        i += 1;
        if i == search_index {
            break current;
        }
        let temp = mem
            .entry(current)
            .and_modify(|x| {
                x.0 = x.1;
                x.1 = i;
            })
            .or_insert((i, i));
        current = temp.1 - temp.0;
    };
    result
}
