use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_5/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    let mut orderings: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut first = true;

    for line in file_lines {
        if line == "" {
            first = false;
            continue;
        }

        if first {
            let mut it = line.split("|").map(|x| x.parse().unwrap());
            let lhs: u32 = it.next().unwrap();
            let rhs: u32 = it.next().unwrap();

            if let Some(coll) = orderings.get_mut(&lhs) {
                coll.push(rhs);
            } else {
                orderings.insert(lhs, vec![rhs]);
            }
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    let sorter = build_sorter_bool(orderings);

    updates
        .iter()
        .filter(|update| {
            println!("checking {:?}", update);
            update.windows(2).all(|x| sorter(x[0], x[1]))
        }).map(|update| update[update.len()/2])
        .sum()
}

fn build_sorter_bool(rules: HashMap<u32, Vec<u32>>) -> impl Fn(u32, u32) -> bool {
    move |a, b| {
        if let Some(smallers) = rules.get(&a) {
            if smallers.contains(&b) {
                return true;
            }
        }
        if let Some(smallers) = rules.get(&b) {
            if smallers.contains(&a) {
                return false;
            }
        }

        return true;
    }
}

fn two(file_lines: Vec<&str>) -> u32 {
    let mut orderings: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut first = true;

    for line in file_lines {
        if line == "" {
            first = false;
            continue;
        }

        if first {
            let mut it = line.split("|").map(|x| x.parse().unwrap());
            let lhs: u32 = it.next().unwrap();
            let rhs: u32 = it.next().unwrap();

            if let Some(coll) = orderings.get_mut(&lhs) {
                coll.push(rhs);
            } else {
                orderings.insert(lhs, vec![rhs]);
            }
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    let sorter_bool = build_sorter_bool(orderings.clone());
    let sorter_ord = build_sorter_ord(orderings);

    updates
        .iter_mut()
        .filter(|update| {
            update.windows(2).any(|x| !sorter_bool(x[0], x[1]))
        }).map(|update|{update.sort_by(|a, b| sorter_ord(*a, *b)); update}).map(|update| update[update.len()/2])
        .sum()
}

fn build_sorter_ord(rules: HashMap<u32, Vec<u32>>) -> impl Fn(u32, u32) -> Ordering {
    move |a, b| {
        if let Some(smallers) = rules.get(&a) {
            if smallers.contains(&b) {
                return Ordering::Less;
            }
        }
        if let Some(smallers) = rules.get(&b) {
            if smallers.contains(&a) {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    }
}
