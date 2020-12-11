use crate::lib::get_input;
use std::cmp::max;

pub fn first() {
    let passes = get_input(5, 1, |l| {
        let (row, column) = l.split_at(7);

        let row = row
            .chars()
            .fold(0, |acc, c| {
                match c {
                    'F' => acc << 1,
                    'B' => (acc << 1) + 1,
                    _ => panic!(format!("{}", c)),
                }
            });

        let column = column
            .chars()
            .fold(0, |acc, c| {
                match c {
                    'L' => acc << 1,
                    'R' => (acc << 1) + 1,
                    _ => panic!(format!("{}", c)),
                }
            });

        (row, column)
    });

    let mut max_id = 0;

    for (row, column) in passes {
        max_id = max(max_id, row * 8 + column);
    }

    dbg!(max_id);
}

pub fn second() {
    let mut passes = get_input(5, 1, |l| {
        let (row, column) = l.split_at(7);

        let row = row
            .chars()
            .fold(0, |acc, c| {
                match c {
                    'F' => acc << 1,
                    'B' => (acc << 1) + 1,
                    _ => panic!(format!("{}", c)),
                }
            });

        let column = column
            .chars()
            .fold(0, |acc, c| {
                match c {
                    'L' => acc << 1,
                    'R' => (acc << 1) + 1,
                    _ => panic!(format!("{}", c)),
                }
            });

        row * 8 + column
    });

    passes.sort();
    passes.dedup();
    let hole = passes.windows(2).map(|s| s[1] - s[0]).position(|d| d == 2).unwrap();

    println!("{}", passes[hole] + 1);
}
