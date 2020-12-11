use std::fs;

pub fn get_input<F, T>(day: u64, n: u64, func: F) -> Vec<T>
    where F: FnMut(&str) -> T
{
    let path = format!("data/day{}/{}", day, n);
    fs::read_to_string(&path)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(func)
        .collect()
}

pub fn get_input_all<F, T>(day: u64, n: u64, func: F) -> Vec<T>
    where F: FnMut(&str) -> T
{
    let path = format!("data/day{}/{}", day, n);
    fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(func)
        .collect()
}
