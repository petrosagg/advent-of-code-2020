use std::fs;
use itertools::Itertools;

pub fn get_input_raw(day: u64, n: u64) -> String {
    let path = format!("data/day{}/{}", day, n);
    fs::read_to_string(&path).unwrap()
}

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

pub fn get_input_filter<F, T>(day: u64, n: u64, func: F) -> Vec<T>
    where F: FnMut(&str) -> Option<T>
{
    let path = format!("data/day{}/{}", day, n);
    fs::read_to_string(&path)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .filter_map(func)
        .collect()
}

pub fn get_input_filter_all<F, T>(day: u64, n: u64, func: F) -> Vec<T>
    where F: FnMut(&str) -> Option<T>
{
    let path = format!("data/day{}/{}", day, n);
    fs::read_to_string(&path)
        .unwrap()
        .lines()
        .filter_map(func)
        .collect()
}

pub fn offsets(dimentions: usize) -> impl Iterator<Item=isize> {
    std::iter::repeat(-1..=1)
        .take(dimentions)
        .multi_cartesian_product()
        .filter(|v| v.iter().any(|e| *e != 0))
        .flatten()
}

