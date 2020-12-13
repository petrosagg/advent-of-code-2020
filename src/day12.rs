use crate::lib::get_input;

static DIRECTIONS: &[(i64, i64); 4] = &[
    ( 1,  0),
    ( 0, -1),
    (-1,  0),
    ( 0,  1),
];

pub fn first() {
    let mut input = get_input(12, 1, |l| l.to_string());

    let mut x = 0i64;
    let mut y = 0i64;
    let mut direction = 0i64;


    for line in input {
        dbg!(&line, x, y, DIRECTIONS[direction as usize]);
        let (command, arg) = line.split_at(1);
        let arg = arg.parse::<i64>().unwrap();

        match command {
            "N" => y += arg,
            "S" => y -= arg,
            "E" => x += arg,
            "W" => x -= arg,
            "F" => {
                x += DIRECTIONS[direction as usize].0 * arg;
                y += DIRECTIONS[direction as usize].1 * arg;
            },
            "L" => {
                direction = (direction - (arg/90) + 4) % 4;
            },
            "R" => {
                direction = (direction + (arg/90) + 4) % 4;
            },
            _ => panic!(),
        }
    }
    println!("{}", x.abs() + y.abs());
}

pub fn second() {
    let mut input = get_input(12, 1, |l| l.to_string());

    let mut w_x = 10i64;
    let mut w_y = 1i64;
    let mut x = 0i64;
    let mut y = 0i64;

    for line in input {
        dbg!(&line, x, y, w_x, w_y);
        let (command, arg) = line.split_at(1);
        let arg = arg.parse::<i64>().unwrap();

        match command {
            "N" => w_y += arg,
            "S" => w_y -= arg,
            "E" => w_x += arg,
            "W" => w_x -= arg,
            "F" => {
                x += (w_x) * arg;
                y += (w_y) * arg;
            },
            "L" => {
                let direction = 4 - ((arg / 90) % 4);
                let mut rel_x = w_x;
                let mut rel_y = w_y;
                match direction {
                    0 => (),
                    1 => {
                        // (3, 4) -> (-4, 3)
                        let tmp = rel_x;
                        rel_x = rel_y;
                        rel_y = -tmp;
                    },
                    2 => {
                        // (3, 4) -> (-4, 3) -> (-3, -4)
                        rel_x = -rel_x;
                        rel_y = -rel_y;
                    },
                    3 => {
                        // (3, 4) -> (-4, 3) -> (-3, -4) -> (-4, 3)
                        let tmp = rel_x;
                        rel_x = -rel_y;
                        rel_y = tmp;
                    },
                    _ => panic!(),
                }
                w_x = rel_x;
                w_y = rel_y;

            },
            "R" => {
                let direction = (arg / 90) % 4;
                let mut rel_x = w_x;
                let mut rel_y = w_y;
                match direction {
                    0 => (),
                    1 => {
                        // (3, 4) -> (-4, 3)
                        let tmp = rel_x;
                        rel_x = rel_y;
                        rel_y = -tmp;
                    },
                    2 => {
                        // (3, 4) -> (-4, 3) -> (-3, -4)
                        rel_x = -rel_x;
                        rel_y = -rel_y;
                    },
                    3 => {
                        // (3, 4) -> (-4, 3) -> (-3, -4) -> (-4, 3)
                        let tmp = rel_x;
                        rel_x = -rel_y;
                        rel_y = tmp;
                    },
                    _ => panic!(),
                }
                w_x = rel_x;
                w_y = rel_y;

            },
            _ => panic!(),
        }
    }
    println!("{}", x.abs() + y.abs());
}
