use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_2/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    file_lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .map(|x| match safe_report(&x) {
            true => 1,
            false => 0,
        })
        .sum::<u32>()
}

fn safe_report(report: &Vec<u32>) -> bool {
    let mut decreasing = false;
    let mut increasing = false;

    for idx in 1..report.len() {
        let prev = report[idx - 1];
        let curr = report[idx];

        if prev == curr {
            return false;
        }
        if curr.abs_diff(prev) > 3 {
            return false;
        }

        let decr = (report[idx] as i32 - report[idx - 1] as i32) < 0;
        if decr {
            if increasing {
                return false;
            } else {
                decreasing = true
            }
        } else {
            if decreasing {
                return false;
            } else {
                increasing = true
            }
        }
    }

    true
}

fn two(file_lines: Vec<&str>) -> u32 {
    file_lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .map(|x| match safer_report(&x) {
            true => 1,
            false => 0,
        })
        .sum::<u32>()
}

fn safer_report(report: &Vec<u32>) -> bool {
    let mut decreasing = false;
    let mut increasing = false;
    let mut prev_failure = false;

    let mut prev = report[0];
    let mut curr = report[1];

    for idx in 1..report.len() {
        if prev == curr {
            if prev_failure{return false;}
        }
        if curr.abs_diff(prev) > 3 {
            return false;
        }

        let decr = (report[idx] as i32 - report[idx - 1] as i32) < 0;
        if decr {
            if increasing {
                return false;
            } else {
                decreasing = true
            }
        } else {
            if decreasing {
                return false;
            } else {
                increasing = true
            }
        }

        prev = report[idx - 1];
        curr = report[idx];
    }

    true
}
