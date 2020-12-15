use std::collections::HashMap;

pub fn first() {
    let input = &[0, 13, 16, 17, 1, 10, 6];

    let mut spoken_at = HashMap::new();

    for (i, n) in input[..input.len()-1].iter().enumerate() {
        spoken_at.insert(*n, i+1);
    }

    let mut last_spoken = (input[input.len()-1], input.len());

    for turn in (input.len()+1)..30000001 {
        let next = match spoken_at.get(&last_spoken.0) {
            Some(position) => (turn - position - 1, turn),
            None => (0, turn),
        };
        spoken_at.insert(last_spoken.0, last_spoken.1);
        last_spoken = next;

        if turn == 2020 || turn == 30000000 {
            println!("Turn {}: {}", turn, last_spoken.0);
        }
    }
}
