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

    let mut ingredient_count = HashMap::new();
    let mut a2i = HashMap::new();

    for (ingredients, allergens) in input {
        for allergen in allergens {
            let l = a2i.entry(allergen).or_insert(vec![]);
            l.push(ingredients.clone());
        }
        for i in ingredients {
            let count = ingredient_count.entry(i.clone()).or_insert(0);
            *count +=1 ;
        }
    }


    let mut possible = HashSet::new();
    for (a, mut ingredients) in a2i.clone() {
        let mut common = ingredients.pop().unwrap();
        common.retain(|el| ingredients.iter().all(|i| i.contains(el)));
        possible.extend(common.into_iter());
    }

    let mut acc = 0;
    for (ingredient, count) in &ingredient_count {
        if possible.contains(ingredient) {
            continue;
        }
        acc += count;
    }
    dbg!(acc);

    let mut a2i2 = HashMap::new();
    for (allergen, mut ingredients) in a2i {
        let mut common = ingredients.pop().unwrap();
        common.retain(|el| ingredients.iter().all(|i| i.contains(el)));

        a2i2.insert(allergen, common);
    }

    let mut res = vec![];
    while a2i2.len() > 0 {
        let (a, i) = a2i2.iter().find(|(k, v)| v.len() == 1).unwrap();
        let i = i.iter().next().unwrap().clone();
        let a = a.clone();
        a2i2.remove(&a);
        for list in a2i2.values_mut() {
            list.remove(&i);
        }
        res.push((a, i));
    }

    res.sort();
    dbg!(res.iter().map(|(_, i)| i).join(","));
}

