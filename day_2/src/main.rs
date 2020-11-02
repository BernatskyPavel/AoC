fn main() {
    let default = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 9, 23, 27, 2,
        27, 6, 31, 1, 5, 31, 35, 2, 9, 35, 39, 2, 6, 39, 43, 2, 43, 13, 47, 2, 13, 47, 51, 1, 10,
        51, 55, 1, 9, 55, 59, 1, 6, 59, 63, 2, 63, 9, 67, 1, 67, 6, 71, 1, 71, 13, 75, 1, 6, 75,
        79, 1, 9, 79, 83, 2, 9, 83, 87, 1, 87, 6, 91, 1, 91, 13, 95, 2, 6, 95, 99, 1, 10, 99, 103,
        2, 103, 9, 107, 1, 6, 107, 111, 1, 10, 111, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123,
        13, 127, 1, 127, 5, 131, 1, 6, 131, 135, 2, 135, 13, 139, 1, 139, 2, 143, 1, 143, 10, 0,
        99, 2, 0, 14, 0,
    ];
    let length = default.len();
    let mut noun = 0;
    let mut verb = 0;
    loop {
        let mut codes = default.clone();
        codes[1] = noun;
        codes[2] = verb;

        //let mut codes = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        for i in 0..codes.len() {
            if i % 4 == 0 {
                let noun_pos= codes[i + 1];
                let verb_pos= codes[i + 2];
                let output_pos= codes[i + 3];
                if noun_pos >= length || verb_pos >= length || output_pos >= length {
                    continue;
                }
                if codes[i] == 1 {
                    codes[output_pos] = codes[noun_pos] + codes[verb_pos];
                } else if codes[i] == 2 {
                    codes[output_pos] = codes[noun_pos] * codes[verb_pos];
                } else if codes[i] == 99 {
                    break;
                }
            }
        }
        if codes[0] == 19690720 || (noun == 99 && verb == 99) {
            break;
        } else {
            if noun == 99 {
                noun = 0;
                verb += 1;
            } else {
                noun += 1;
            }
        }
    }

    
    println!("{}-{}-{}", noun, verb, 100*noun+verb);
    //println!("{}", codes[0]);
}
