use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_1/input_1";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    let mut lhs_list = Vec::<u32>::new();
    let mut rhs_list = Vec::<u32>::new();

    let re = Regex::new(r"(?<lhs>\d+)\s+(?<rhs>\d+)").unwrap();
    file_lines.iter().for_each(|line| {
        lhs_list.push(
            re.captures(line)
                .unwrap()
                .name("lhs")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        );
        rhs_list.push(
            re.captures(line)
                .unwrap()
                .name("rhs")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        );
    });

    lhs_list.sort();
    rhs_list.sort();

    zip(lhs_list, rhs_list)
        .map(|(x, y)| {
            let diff = x.abs_diff(y);
            diff
        })
        .sum()
}

fn two(file_lines: Vec<&str>) -> u32 {
    let mut lhs_list = Vec::<u32>::new();
    let mut rhs_map = HashMap::<u32, u32>::new();

    let re = Regex::new(r"(?<lhs>\d+)\s+(?<rhs>\d+)").unwrap();
    file_lines.iter().for_each(|line| {
        let lhs_num = re
            .captures(line)
            .unwrap()
            .name("lhs")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let rhs_num = re
            .captures(line)
            .unwrap()
            .name("rhs")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        lhs_list.push(lhs_num);
        rhs_map.insert(rhs_num, rhs_map.get(&rhs_num).unwrap_or(&0) + 1);
    });

    lhs_list
        .iter()
        .map(|left| rhs_map.get(left).unwrap_or(&0) * left)
        .sum()
}
