use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_3/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    mul_commands(&file_lines)
}

fn two(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    mul_commands(&[new_lines.as_str()])
}

fn mul_commands(lines: &[&str]) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();

    lines
        .iter()
        .map(|line| {
            re.captures_iter(line)
                .map(|m| {
                    m.name("lhs").unwrap().as_str().parse::<u32>().unwrap()
                        * m.name("rhs").unwrap().as_str().parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .sum()
}
