use std::collections::{HashMap};
use itertools::Itertools;

use crate::lib::*;

#[derive(Debug,Clone)]
enum Rule {
    Or(Vec<Rule>),
    And(Vec<Rule>),
    Ref(usize),
    Lit(char),
}

fn parse<'a>(input: &'a str, rule: &Rule, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    match rule {
        Rule::And(ands) => {
            let mut rest = input;
            for rule in ands {
                rest = parse(rest, rule, rules)?
            }
            Some(rest)
        }
        Rule::Or(ors) => {
            for and in ors {
                if let Some(rest) = parse(input, and, rules) {
                    return Some(rest)
                }
            }
            None
        }
        Rule::Ref(idx) => {
            // println!("ref {}: input={}", idx, input);
            parse(input, rules.get(idx).unwrap(), rules)
        }
        Rule::Lit(c) => {
            // println!("lit {}: input={}", c, input);
            if input.starts_with(*c) {
                // println!("\tmatch");
                Some(&input[1..])
            } else {
                None
            }
        }
    }
}

pub fn first() {
    let input = get_input_raw(19, 2);
    let input = input.split("\n\n").collect_vec();

    let rules: HashMap<_, _> = input[0]
        .lines()
        .map(|l| {
            let idx = l.find(':').unwrap() + 2;
            let (ord, rule) = l.split_at(idx);
            let ord = ord[..ord.len()-2].parse::<usize>().unwrap();

            (ord, Rule::Or(rule
                .split(" | ")
                .map(|rule| {
                    Rule::And(rule
                        .split(' ')
                        .map(|rule| {
                            if rule.starts_with('"') {
                                Rule::Lit(rule.chars().nth(1).unwrap())
                            } else {
                                Rule::Ref(rule.parse().unwrap())
                            }
                        })
                        .collect_vec()
                    )
                })
                .collect_vec()
            ))
        })
        .collect();

    dbg!(&rules);
    let mut acc = 0;
    for input in input[1].lines() {
        println!("input={}", input);
        if Some("") == parse(input, rules.get(&0).unwrap(), &rules) {
            acc += 1;
        }
    }
    dbg!(acc);
}
