use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::lib::*;

fn solve(dimentions: usize) {
    let layer = get_input(17, 1, |l| {
        l.chars().map(|c| c == '#').collect_vec()
    });

    let mut cubes = HashSet::new();
    let mut next_cubes = HashSet::new();
    let mut neighbours = HashMap::new();

    for (y, row) in layer.iter().enumerate() {
        for (x, active) in row.iter().enumerate() {
            if *active {
                cubes.insert((x as isize, y as isize, 0 as isize, 0 as isize));
            }
        }
    }

    for _ in 0..6 {
        for (x, y, z, w) in &cubes {
            for (dx, dy, dz, dw) in offsets(dimentions).tuples() {
                let coord = (*x + dx, *y + dy, *z + dz, *w + dw);
                let n = neighbours.entry(coord).or_insert(0);
                *n += 1;
            }
        }

        for (coord, n) in neighbours.drain() {
            if n == 3 || (n == 2 && cubes.contains(&coord)) {
                next_cubes.insert(coord);
            }
        }

        cubes.clear();
        std::mem::swap(&mut cubes, &mut next_cubes);
    }

    dbg!(cubes.len());
}

pub fn first() {
    solve(3);
}

pub fn second() {
    solve(4);
}
