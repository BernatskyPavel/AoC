use std::collections::BTreeSet;

fn main() {
    let pass = part_one(String::from("cqjxjnds"));
    println!("Part one: {}", pass);
    println!("Part two: {}", part_one(pass));
}

struct Password {
    symbols: Vec<char>,
}

impl Password {
    fn new(password: String) -> Option<Password> {
        if password.len() != 8 {
            None
        } else {
            Some(Password {
                symbols: password.chars().collect(),
            })
        }
    }

    fn increment_password(&mut self) {
        let mut is_done;
        let mut i = 7;
        loop {
            if self.symbols[i] as u8 == 122 {
                self.symbols[i] = 97_u8 as char;
                is_done = false;
            } else {
                self.symbols[i] = (self.symbols[i] as u8 + 1) as char;
                is_done = true;
            }
            if is_done {
                break;
            }
            if i == 0 {
                i = 7;
            } else {
                i -= 1;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut is_valid = self.symbols.windows(3).any(|window| {
            window[0] as u8 == (window[1] as u8 - 1) && window[1] as u8 == (window[2] as u8 - 1)
        });
        if is_valid {
            is_valid = self.symbols.iter().all(|ch| !"iol".contains(*ch));
        }
        if is_valid {
            let pairs = self
                .symbols
                .windows(2)
                .filter(|window| window[0] == window[1])
                .collect::<BTreeSet<&[char]>>();
            if pairs.len() < 2 {
                is_valid = false;
            }
        }
        is_valid
    }

    fn get_password(&self) -> String {
        self.symbols.iter().collect()
    }
}

fn part_one(old_password: String) -> String {
    if let Some(mut new_password) = Password::new(old_password) {
        new_password.increment_password();
        while !new_password.is_valid() {
            new_password.increment_password();
        }
        new_password.get_password()
    } else {
        String::new()
    }
}
