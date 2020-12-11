use crate::lib::get_input;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn first() -> i64 {
    let v = get_input(9, 1, |l| l.parse::<i64>().unwrap());

    let (preamble, data) = v.split_at(25);

    let mut prefix_set: HashSet<i64> = HashSet::new();
    let mut prefix: VecDeque<i64> = VecDeque::new();

    prefix.extend(preamble);
    prefix_set.extend(preamble);

    for n in data {
        let valid = prefix.iter().any(|a| prefix_set.contains(&(n - a)));
        if !valid {
            println!("invalid {}", n);
            return *n;
        }
        let last = prefix.pop_front().unwrap();
        prefix_set.remove(&last);
        prefix.push_back(*n);
        prefix_set.insert(*n);
    }

    unreachable!();
}


pub fn second() {
    let v = get_input(9, 1, |l| l.parse::<i64>().unwrap());

    let mut prefix_sum = vec![0; v.len()];
    prefix_sum[0] = v[0];

    for i in 1..v.len() {
        prefix_sum[i] = v[i] + prefix_sum[i-1];
    }

    let mut sums: HashMap<i64, _> = HashMap::new();
    sums.extend(prefix_sum.clone().into_iter().zip(1..));

    let invalid = first();

    for (a_sum, a_idx) in prefix_sum.into_iter().zip(1..) {
        if let Some(b_idx) = sums.get(&(a_sum - invalid)) {
            let mut region = v[(*b_idx as usize)..(a_idx as usize)].to_vec();
            region.sort();
            println!("{}", region[0] + region[region.len() - 1 ]);
            break;
        }
    }
}
