use crate::lib::get_input;
use std::fmt;

#[derive(PartialEq,Eq,Clone)]
enum State {
    Floor,
    Occupied,
    Empty,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Floor => f.write_str("."),
            State::Empty => f.write_str("L"),
            State::Occupied => f.write_str("#"),
        }
    }
}

static DIRECTIONS: &[(i64, i64); 8] = &[
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

pub fn first() {
    let mut seats = get_input(11, 1, |l| l.chars().map(|c| {
        match c {
            'L' => State::Empty,
            '.' => State::Floor,
            '#' => State::Occupied,
            _ => panic!(),
        }
    }).collect::<Vec<_>>());

    let grid_x = seats[0].len() as i64;
    let grid_y = seats.len() as i64;

    let mut unstable = true;
    while unstable {
        let mut new_seats = seats.clone();

        unstable = false;
        for (y, row) in seats.iter().enumerate() {
            for (x, state) in row.into_iter().enumerate() {
                let mut adjacent = 0;
                for (i, j) in DIRECTIONS {
                    let x_a = x as i64 + i;
                    let y_a = y as i64 + j;
                    if x_a >= 0 && x_a < grid_x && y_a >= 0 && y_a < grid_y {
                        if seats[y_a as usize][x_a as usize] == State::Occupied {
                            adjacent += 1;
                        }
                    }
                }
                if *state == State::Occupied && adjacent >= 4 {
                    new_seats[y][x] = State::Empty;
                    unstable = true;
                }
                if *state == State::Empty && adjacent == 0 {
                    new_seats[y][x] = State::Occupied;
                    unstable = true;
                }
            }
        }

        seats = new_seats;
    }

    let mut occupied = 0;

    for row in seats {
        for seat in row {
            if seat == State::Occupied {
                occupied += 1;
            }
        }
    }
    println!("{}", occupied);
}

pub fn second() {
    let mut seats = get_input(11, 1, |l| l.chars().map(|c| {
        match c {
            'L' => State::Empty,
            '.' => State::Floor,
            '#' => State::Occupied,
            _ => panic!(),
        }
    }).collect::<Vec<_>>());

    let grid_x = seats[0].len() as i64;
    let grid_y = seats.len() as i64;

    let mut unstable = true;
    while unstable {
        let mut new_seats = seats.clone();

        unstable = false;
        for (y, row) in seats.iter().enumerate() {
            for (x, state) in row.into_iter().enumerate() {
                let mut adjacent = 0;
                for (i, j) in DIRECTIONS {
                    let mut x_a = x as i64 + i;
                    let mut y_a = y as i64 + j;
                    while x_a >= 0 && x_a < grid_x && y_a >= 0 && y_a < grid_y {
                        match seats[y_a as usize][x_a as usize] {
                            State::Floor => (),
                            State::Occupied => {
                                adjacent += 1;
                                break;
                            }
                            State::Empty => break,
                        }
                        x_a += i;
                        y_a += j;
                    }
                }
                if *state == State::Occupied && adjacent >= 5 {
                    new_seats[y][x] = State::Empty;
                    unstable = true;
                }
                if *state == State::Empty && adjacent == 0 {
                    new_seats[y][x] = State::Occupied;
                    unstable = true;
                }
            }
        }

        seats = new_seats;
    }

    let mut occupied = 0;

    for row in seats {
        for seat in row {
            if seat == State::Occupied {
                occupied += 1;
            }
        }
    }
    println!("{}", occupied);
}
