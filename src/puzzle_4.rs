use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_4/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    LR,
    RL,
    UD,
    DU,
    UL,
    UR,
    DR,
    DL,
}

fn one(file_lines: Vec<&str>) -> u32 {
    let mut sum = 0;

    let columns = file_lines[0].len();
    let rows = file_lines.len();

    let xmas = vec!["X", "M", "A", "S"];

    let directions = |size: usize| {
        HashMap::from([
            (
                Direction::LR,
                (0..size)
                    .map(|i| (i as i32, 0))
                    .collect::<Vec<(i32, i32)>>(),
            ),
            (Direction::RL, (0..size).map(|i| (-(i as i32), 0)).collect()),
            (Direction::UD, (0..size).map(|i| (0, i as i32)).collect()),
            (Direction::DU, (0..size).map(|i| (0, -(i as i32))).collect()),
            (
                Direction::UL,
                (0..size).map(|i| (-(i as i32), -(i as i32))).collect(),
            ),
            (
                Direction::UR,
                (0..size).map(|i| (i as i32, -(i as i32))).collect(),
            ),
            (
                Direction::DR,
                (0..size).map(|i| (i as i32, i as i32)).collect(),
            ),
            (
                Direction::DL,
                (0..size).map(|i| (-(i as i32), i as i32)).collect(),
            ),
        ])
    };

    let xmas_search = directions(xmas.len());

    xmas_search.into_values().map(
        |offsets| {
            for (x, y) in offsets.iter().map(|(xo, yo)| (0 + xo, 0 + yo)) {
                let (x, y) = (x as usize, y as usize);
                file_lines.get(y).unwrap_or(&"").get(x..x+1).unwrap();
            }
        }
    );

    let d_u_l = |arr: &[&str], (x, y): (usize, usize)| {
        if y < 3 || x < 3 {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y - i].get(x - i..x - i + 1).unwrap() == xmas[i])
            })
        }
    };

    let check_all = |arr: &[&str], (x, y): (usize, usize)| -> u32 {
        let mut out = 0;
        if h_l_r(arr, (x, y)) {
            out += 1;
        }
        if h_r_l(arr, (x, y)) {
            out += 1;
        }
        if v_u_d(arr, (x, y)) {
            out += 1;
        }
        if v_d_u(arr, (x, y)) {
            out += 1;
        }
        if d_u_r(arr, (x, y)) {
            out += 1;
        }
        if d_d_r(arr, (x, y)) {
            out += 1;
        }
        if d_d_l(arr, (x, y)) {
            out += 1;
        }
        if d_u_l(arr, (x, y)) {
            out += 1;
        }
        out
    };

    for i in 0..columns {
        for j in 0..rows {
            sum += check_all(&file_lines, (i, j));
        }
    }

    sum
}

fn two(file_lines: Vec<&str>) -> u32 {
    let mut sum = 0;

    let columns = file_lines[0].len();
    let rows = file_lines.len();

    let x = |arr: &[&str], (x, y): (usize, usize)| {
        if x == 0 || y == 0 {
            return false;
        }

        let mid = arr.get(y).unwrap_or(&"").get(x..x + 1);

        if mid != Some("A") {
            return false;
        }

        let u_l = arr
            .get(y.saturating_sub(1))
            .unwrap_or(&"")
            .get(x.saturating_sub(1)..x)
            .unwrap_or_default();
        let d_l = arr
            .get(y + 1)
            .unwrap_or(&"")
            .get(x.saturating_sub(1)..x)
            .unwrap_or_default();
        let d_r = arr
            .get(y + 1)
            .unwrap_or(&"")
            .get(x + 1..x + 2)
            .unwrap_or_default();
        let u_r = arr
            .get(y.saturating_sub(1))
            .unwrap_or(&"")
            .get(x + 1..x + 2)
            .unwrap_or_default();

        if u_l == d_r || u_r == d_l {
            return false;
        }

        let charbok = vec![u_l, d_l, u_r, d_r];

        let num_m = charbok.iter().filter(|&&x| x == "M").count();
        let num_s = charbok.iter().filter(|&&x| x == "S").count();

        if num_m == 2 && num_s == 2 {
            return true;
        } else {
            return false;
        }
    };

    for i in 0..columns {
        for j in 0..rows {
            if x(&file_lines, (i, j)) {
                sum += 1;
            }
        }
    }

    sum
}
