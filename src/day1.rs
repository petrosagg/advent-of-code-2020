use crate::lib::get_input;
use std::collections::HashSet;

pub fn first() {
    let input = get_input(1, 1, |l| l.parse::<u64>().unwrap());

    let mut seen = HashSet::new();

    for n in input {
        if seen.contains(&(2020 - n)) {
            println!("{}", n * (2020 - n));
        }
        seen.insert(n);
    }
}

pub fn second() {
    let input = get_input(1, 1, |l| l.parse::<u64>().unwrap());

    for (i, a) in input.iter().enumerate() {
        for (j, b) in input[i..].iter().enumerate() {
            for c in input[j..].iter() {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}

