use std::{collections::HashMap, fs::read_to_string, iter::zip};

use itertools::Itertools;
use regex::Regex;

pub fn solve() -> u64 {
    let filepath = "assets/puzzle_9/input.txt";
    one(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    file_lines.iter().for_each(|line| {
        let indices = line.len() / 2 + 1;
        let mut files = Vec::<u64>::new();
        let mut spaces = Vec::new();
        line.char_indices().for_each(|(i, c)| {
            if i % 2 == 0 {
                files.push(c.to_digit(10).unwrap() as u64);
            } else {
                spaces.push(c.to_digit(10).unwrap());
            }
        });

        let filespace: u64 = files.iter().sum();
        let mut vals = Vec::with_capacity(filespace as usize);

        files
            .iter()
            .enumerate().take_while(|(i, _)| i<&(filespace as usize))
            .for_each(|(i, size)| vals.extend((0..*size).map(|_| i)));

        let mut vals_forward = vals.iter();
        let mut vals_backwards = vals.iter().rev();
        let mut idx = 0;

        line.char_indices().for_each(|(i, c)| {
            let size = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                (0..size).for_each(|_| {
                    if let Some(v) = vals_forward.next() {
                        if idx >= filespace {
                            return;
                        }
                        sum += *v as u64 * idx;
                        idx += 1;
                    }
                });
            } else {
                (0..size).for_each(|_| {
                    if let Some(v) = vals_backwards.next() {
                        if idx >= filespace {
                            return;
                        }
                        sum += *v as u64 * idx;
                        idx += 1;
                    }
                });
            }
        });
    });

    sum as u64
    // produced 1408072414 very slowly lol
}

fn two(file_lines: Vec<&str>) -> u64 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    0
}
