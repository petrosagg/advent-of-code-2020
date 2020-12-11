use crate::lib::get_input;

#[derive(Debug)]
struct Policy {
    c: char,
    low: usize,
    high: usize,
}

fn parse(line: &str) -> (Policy, String) {
    let mut fields = line.split(' ');
    let range = fields.next().unwrap();
    let c = fields.next().unwrap().chars().next().unwrap();
    let password = fields.next().unwrap();

    let mut range_fields = range.split('-');
    let low = range_fields.next().unwrap().parse().unwrap();
    let high = range_fields.next().unwrap().parse().unwrap();

    let policy = Policy { c, low, high };

    (policy, password.to_string())
}

pub fn first() {
    let input = get_input(2, 1, parse);

    let valid = input.iter().filter(|(policy, password)| {
        let count = password.chars().filter(|c| *c == policy.c).count();

        policy.low <= count && count <= policy.high
    }).count();

    println!("{}", valid);
}

pub fn second() {
    let input = get_input(2, 1, parse);

    let valid = input.iter().filter(|(policy, password)| {
        let count = password
            .chars()
            .enumerate()
            .filter(|(i, c)| {
                let i = i + 1;
                (i == policy.low || i == policy.high) && *c == policy.c
            }).count();

        count == 1
    }).count();

    println!("{}", valid);
}
