use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::lib::*;

pub fn first() {
    let input = get_input(21, 1, |l| {
        let parts = l[..l.len()-1].split(" (contains ").collect_vec();
        let ingredients: HashSet<_> = parts[0].split(' ').map(|s| s.to_string()).collect();
        let allergens = parts[1].split(", ").map(|s| s.to_string()).collect_vec();
        (ingredients, allergens)
    });

    let mut ingredient_count: HashMap<&str, u64> = HashMap::new();
    let mut possible_ingredients: HashMap<&str, HashSet<String>> = HashMap::new();

    for (ingredients, allergens) in &input {
        for i in ingredients {
            let count = ingredient_count.entry(i).or_default();
            *count +=1;
        }

        for allergen in allergens {
            possible_ingredients
                .entry(allergen)
                .and_modify(|common: &mut HashSet<_>| common.retain(|el| ingredients.contains(el)))
                .or_insert_with(|| ingredients.clone());
        }
    }

    let mut possible: HashSet<&str> = HashSet::new();

    for common in possible_ingredients.values() {
        possible.extend(common.iter().map(|x| x.as_str()));
    }

    let solution1: u64 = ingredient_count.iter()
        .filter(|(i, _)| !possible.contains(*i))
        .map(|(_, c)| c)
        .sum();

    dbg!(solution1);

    let mut res = vec![];
    while possible_ingredients.len() > 0 {
        let (a, i) = possible_ingredients.iter().find(|(_, v)| v.len() == 1).unwrap();
        let i = i.iter().next().unwrap().clone();
        let a = a.clone();
        possible_ingredients.remove(&a);
        for list in possible_ingredients.values_mut() {
            list.remove(&i);
        }
        res.push((a, i));
    }
    res.sort();

    let solution2 = res.iter().map(|(_, i)| i).join(",");
    dbg!(solution2);
}

