use std::{collections::HashMap, f32::consts::PI, fs::read_to_string, iter::zip};

use itertools::Itertools;
use regex::Regex;

pub fn solve() -> u64 {
    let filepath = "assets/puzzle_7/input";
    one(read_to_string(filepath).unwrap().lines().collect())
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn one(file_lines: Vec<&str>) -> u64 {
    let base = vec![Op::Add, Op::Mul];
    let b = base.len();

    file_lines
        .iter()
        .map(|line| {
            let mut it = line.split(": ");
            let total: u64 = it.next().unwrap().parse().unwrap();
            let pieces: Vec<_> = it
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (total, pieces)
        })
        .map(|(total, pieces)| {
            if all_possible_permutations(&base, (pieces.len() - 1) as u32)
                .iter()
                .any(|opers| {
                    let val = opers
                        .iter()
                        .enumerate()
                        .fold(pieces[0], |acc, (i, &o)| match o {
                            Op::Add => acc + pieces[i + 1],
                            Op::Mul => acc * pieces[i + 1],
                        });

                    val == total
                })
            {
                total
            } else {
                0
            }
        })
        .sum()
    // produced 2654749936343
}

fn all_possible_permutations<T>(vals: &[T], n: u32) -> Vec<Vec<T>>
where
    T: Copy,
{
    let base = vals.len();
    (0..base.pow(n))
        .map(|x| -> Vec<T> {
            (0..n)
                .map(|y| vals[(x / base.pow(y as u32)) % base])
                .collect()
        })
        .collect()
}

fn two(file_lines: Vec<&str>) -> u64 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    0
}
