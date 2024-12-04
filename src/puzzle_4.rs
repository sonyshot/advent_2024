use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_4/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    let mut sum = 0;

    let columns = file_lines[0].len();
    let rows = file_lines.len();

    let xmas = vec!["X", "M", "A", "S"];

    let h_l_r = |arr: &[&str], (x, y): (usize, usize)| {
        if x + 4 > columns {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y].get(x + i..x + i + 1).unwrap() == xmas[i])
            })
        }
    };
    let h_r_l = |arr: &[&str], (x, y): (usize, usize)| {
        if x < 3 {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y].get(x - i..x - i + 1).unwrap() == xmas[i])
            })
        }
    };
    let v_u_d = |arr: &[&str], (x, y): (usize, usize)| {
        if y + 4 > rows {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y + i].get(x..x + 1).unwrap() == xmas[i])
            })
        }
    };
    let v_d_u = |arr: &[&str], (x, y): (usize, usize)| {
        if y < 3 {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y - i].get(x..x + 1).unwrap() == xmas[i])
            })
        }
    };

    let d_u_r = |arr: &[&str], (x, y): (usize, usize)| {
        if y < 3 || x + 4 > columns {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y - i].get(x + i..x + i + 1).unwrap() == xmas[i])
            })
        }
    };
    let d_d_r = |arr: &[&str], (x, y): (usize, usize)| {
        if y + 4 > rows || x + 4 > columns {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y + i].get(x + i..x + i + 1).unwrap() == xmas[i])
            })
        }
    };
    let d_d_l = |arr: &[&str], (x, y): (usize, usize)| {
        if y + 4 > rows || x < 3 {
            false
        } else {
            (0..4).fold(true, |acc, i| {
                acc && (arr[y + i].get(x - i..x - i + 1).unwrap() == xmas[i])
            })
        }
    };
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
