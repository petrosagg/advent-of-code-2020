use crate::lib::get_input;

pub fn first() {
    let mut adapters = get_input(10, 2, |l| l.parse::<u64>().unwrap());
    adapters.push(0);
    adapters.sort();

    let (ones, threes) = adapters
        .windows(2)
        .map(|s| s[1] - s[0])
        .fold((0, 1), |(ones, threes), diff| {
            match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }
        });

    println!("{}", ones * threes);
}

pub fn second() {
    let mut adapters = get_input(10, 2, |l| l.parse::<u64>().unwrap());
    adapters.push(0);
    adapters.sort();

    let mut dp = vec![0u64; adapters.len()];

    dp[0] = 1;
    for i in 1..adapters.len() {
        let mut ways = dp[i-1];
        if i >= 2 {
            if adapters[i] - adapters[i-2] < 4 {
                ways += dp[i-2];
            }
        }
        if i >= 3 {
            if adapters[i] - adapters[i-3] < 4 {
                ways += dp[i-3];
            }
        }
        dp[i] = ways;
    }

    println!("{}", dp[adapters.len() - 1]);
}
