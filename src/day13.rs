use crate::lib::get_input;

pub fn first() {
    let mut input = get_input(13, 1, |l| l.to_string());

    let earliest = input[0].parse::<i64>().unwrap();
    let buses = input[1].split(',').filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<_>>();

    let mut min_offset = i64::MAX;
    let mut min_id = 0;
    for id in buses {
        let f = earliest / id + 1;
        let offset = f * id - earliest;

        if offset < min_offset {
            min_offset = offset;
            min_id = id;
        }
    }

    dbg!(min_id * min_offset);
}

pub fn second() {
    let mut input = get_input(13, 1, |l| l.to_string());

    let buses = input[1]
        .split(',')
        .filter_map({
            let mut skip = 1;

            move |x| {
                match x {
                    "x" => {
                        skip += 1;
                        None
                    }
                    _ => {
                        let ret = Some((x.parse::<u64>().unwrap(), skip));
                        skip = 1;
                        ret
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let mut offset = 1u64;
    let mut multiple = 1u64;
    let mut total_diff = 0;
    let mut best_guess = 0;
    for s in buses.windows(2) {
        let (id, _) = s[0];
        let (next_id, diff) = s[1];
        total_diff += diff;
        // find the first multiple of id that is `diff` units before next_id
        for i in 0.. {
            let n = id * (offset + multiple * i);
            if n % next_id == next_id - diff {
                offset = (n + diff) / next_id;
                multiple *= id;
                best_guess = offset * next_id - total_diff;
                dbg!(n, diff);
                dbg!(best_guess);
                println!("found multiple for {}: {}", next_id, (n + diff) / next_id);
                break
            }
        }
    }
}
