fn main() {
    println!("Hello, world!");
    part_one(String::from("1113222113"), 50);
}

fn part_one(input: String, iterations: usize) {
    let mut result = input;
    (0..iterations).for_each(|_| {
        let mut temp: String = String::new();
        let mut prev = result.chars().nth(0).unwrap();
        let mut counter = 0;
        result.chars().for_each(|ch|{
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
    println!("{}", result.len());
}
