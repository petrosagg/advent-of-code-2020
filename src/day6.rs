use crate::lib::get_input_all;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn first() {
    let answers = get_input_all(6, 1, |l| l.to_string());

    let mut total = 0;
    let mut group = HashSet::new();
    for line in answers {
        if line.is_empty() {
            total += group.len();
            group.clear();
        } else {
            group.extend(line.chars());
        }
    }
    println!("{}", total);
}

pub fn second() {
    let answers = get_input_all(6, 1, |l| l.to_string());

    let mut total = 0;
    let mut group_size = 0;
    let mut group = HashMap::new();
    for line in answers {
        if line.is_empty() {
            total += group.values().filter(|v| **v == group_size).count();
            group.clear();
            group_size = 0;
        } else {
            for c in line.chars() {
                let entry = group.entry(c).or_insert(0);
                *entry += 1;
            }
            group_size += 1;
        }
    }
    println!("{}", total);
}
