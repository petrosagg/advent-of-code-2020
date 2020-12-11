use crate::lib::get_input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Rule {
    bag: String,
    contents: HashMap<String, u64>,
}

fn parse_rules(l: &str) -> Rule {
    let mut l = l.split(" bags contain ");
    let bag = l.next().unwrap().to_string();
    let contents = l.next().unwrap()
        .split(", ")
        .map(|c| c.trim_matches('.'))
        .filter(|c| *c != "no other bags")
        .map(|c| {
            let mut iter = c.split(' ');
            let n = iter.next().unwrap().parse::<u64>().unwrap();
            let bag = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
            (bag, n)
        })
        .collect::<HashMap<_, _>>();

    Rule {
        bag,
        contents,
    }
}

pub fn first() {
    let rules = get_input(7, 1, parse_rules);

    let mut parents = HashMap::new();

    for rule in &rules {
        for child in rule.contents.keys() {
            parents.entry(child).or_insert_with(|| vec![]).push(&rule.bag);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back("shiny gold".to_string());

    let mut bags = HashSet::new();

    while let Some(bag) = queue.pop_front() {
        if let Some(parents) = parents.get(&bag) {
            for p in parents {
                bags.insert(p);
                queue.push_back(p.to_string());
            }
        }
    }

    dbg!(bags.len());
}

fn bag_count(bag: &String, rules: &HashMap<&String, Vec<(&String, &u64)>>) -> u64 {
    let mut total = 1;

    if let Some(children) = rules.get(bag) {
        for (child, n) in children {
            total += bag_count(child, rules) * *n;
        }
    }

    total
}

pub fn second() {
    let rules_vec = get_input(7, 1, parse_rules);

    let mut rules = HashMap::new();

    for rule in &rules_vec {
        let rules = rules.entry(&rule.bag).or_insert_with(|| vec![]);
        for child in &rule.contents {
            rules.push(child);
        }
    }

    let count = bag_count(&"shiny gold".to_string(), &rules);
    dbg!(count - 1);
}
