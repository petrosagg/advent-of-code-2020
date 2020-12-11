use crate::lib::get_input;

pub fn first() {
    let input = get_input(3, 1, |l| l.chars().map(|c| c == '#').collect::<Vec<_>>());


    let mut trees = 0;

    for (i, row) in input.iter().enumerate().skip(1) {
        if row[(i * 3) % row.len()] {
            trees += 1;
        }
    }

    println!("{}", trees);
}

pub fn second() {
    let input = get_input(3, 1, |l| l.chars().map(|c| c == '#').collect::<Vec<_>>());

    let patterns = &[
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let trees: u64 = patterns
        .iter()
        .map(|(right, down)| {
            let mut trees = 0;
            for (step, row) in input.iter().step_by(*down).enumerate().skip(1) {
                if row[(step * *right) % row.len()] {
                    trees += 1;
                }
            }
            trees
        })
        .fold(1, |acc, x| acc * x);

    println!("{}", trees);
}
