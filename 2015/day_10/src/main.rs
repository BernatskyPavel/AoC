fn main() {
    let input = "1113222113";
    println!("Part one: {}", part_one(input, 40));
    println!("Part two: {}", part_one(input, 50));
}

fn part_one(input: &str, iterations: usize) -> usize {
    let mut result = String::from(input);
    (0..iterations).for_each(|_| {
        let mut temp: String = String::new();
        let mut prev = result.chars().next().unwrap();
        let mut counter = 0;
        result.chars().for_each(|ch| {
            if ch == prev {
                counter += 1;
            } else {
                temp.push_str(counter.to_string().as_str());
                temp.push(prev);
                prev = ch;
                counter = 1;
            }
        });
        temp.push_str(counter.to_string().as_str());
        temp.push(prev);
        result = temp.clone();
    });
    result.len()
}
