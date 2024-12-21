use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u64 {
    let filepath = "assets/puzzle_11/input.txt";
    two(read_to_string(filepath).unwrap().lines())
}

fn one<'a>(file_lines: impl Iterator<Item = &'a str>) -> u64 {
    let mut starting_stones: HashMap<u64, u64> = file_lines
    .map(|line| {
        line.split(' ')
            .map(|n| (n.parse().unwrap(), 1))
            .collect()
    })
    .next()
    .unwrap();

    blink_repeatedly(starting_stones, 25).values().sum()
}

fn two<'a>(file_lines: impl Iterator<Item = &'a str>) -> u64 {
    let starting_stones: HashMap<u64, u64> = file_lines
    .map(|line| {
        line.split(' ')
            .map(|n| (n.parse().unwrap(), 1))
            .collect()
    })
    .next()
    .unwrap();

    blink_repeatedly(starting_stones, 75).values().sum()
}

fn blink(val: u64) -> (u64, Option<u64>) {
    if val == 0 {
        (1, None)
    } else if val.checked_ilog10().unwrap() % 2 == 1 {
        let exponent = val.checked_ilog10().unwrap() / 2 + 1;
        let chopped = split_num(val, 10_u64.pow(exponent));
        (chopped.0, Some(chopped.1))
    } else {
        (val * 2024, None)
    }
}

fn blink_repeatedly(mut starting_stones: HashMap<u64, u64>, count: usize) -> HashMap<u64, u64> {
    for _ in 0..count {
        let mut intermediate = HashMap::new();
        starting_stones.drain().for_each(|(k, v)|{
            let blinked = blink(k);
            intermediate.insert(blinked.0, intermediate.get(&blinked.0).unwrap_or(&0)+v);
            if let Some(n) = blinked.1{
                intermediate.insert(n, intermediate.get(&n).unwrap_or(&0)+v);
            }
        });
        starting_stones = intermediate;
    }

    starting_stones
}

fn split_num(n: u64, divisor: u64) -> (u64, u64) {
    (n / divisor, n % divisor)
}
