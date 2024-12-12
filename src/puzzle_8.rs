use itertools::{iproduct, Itertools};
use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::zip,
};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_8/input";
    two(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"[^\.]").unwrap();
    let mut antennas: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

    let cols = file_lines.len() as i32;
    let rows = file_lines[0].len() as i32;

    file_lines.iter().enumerate().for_each(|(i, &line)| {
        re.captures_iter(line).enumerate().for_each(|x| {
            let mat = x.1.get(0).unwrap();
            let pos = (mat.start() as i32, i as i32);
            if let Some(antenna) = antennas.get_mut(mat.as_str()) {
                antenna.push(pos);
            } else {
                antennas.insert(mat.as_str(), vec![pos]);
            }
        });
    });

    let mut anodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennas.iter() {
        for (p1, p2) in iproduct!(positions, positions) {
            if p1 == p2 {
                continue;
            }

            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;

            let anode_pos1 = (p1.0 - dx, p1.1 - dy);
            let anode_pos2 = (p2.0 + dx, p2.1 + dy);

            if 0 <= anode_pos1.0 && anode_pos1.0 < cols && 0 <= anode_pos1.1 && anode_pos1.1 < rows
            {
                anodes.insert(anode_pos1);
            }

            if 0 <= anode_pos2.0 && anode_pos2.0 < cols && 0 <= anode_pos2.1 && anode_pos2.1 < rows
            {
                anodes.insert(anode_pos2);
            }
        }
    }

    anodes.len() as u32
}

fn two(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"[^\.]").unwrap();
    let mut antennas: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

    let cols = file_lines.len() as i32;
    let rows = file_lines[0].len() as i32;

    file_lines.iter().enumerate().for_each(|(i, &line)| {
        re.captures_iter(line).enumerate().for_each(|x| {
            let mat = x.1.get(0).unwrap();
            let pos = (mat.start() as i32, i as i32);
            if let Some(antenna) = antennas.get_mut(mat.as_str()) {
                antenna.push(pos);
            } else {
                antennas.insert(mat.as_str(), vec![pos]);
            }
        });
    });

    let mut lines = HashSet::new();
    let eps = 0.00001;
    for (_, positions) in antennas.iter() {
        
        positions.iter().combinations(2).for_each(|combo| {
            let p1 = combo[0];
            let p2 = combo[1];
            let dy = p2.1 - p1.1;
            let dx = p2.0 - p1.0;

            let m = dy as f32 / dx as f32;
            let b = p2.1 as f32 - m*p2.0 as f32;

            for i in 0..cols {
                let y_val = m*(i as f32)+b;

                if y_val < 0.0 || y_val >= rows as f32 {
                    continue;
                }

                if (y_val.round() - y_val).abs() <= eps{
                    lines.insert((i, y_val.round() as i32));
                }
            }
        });
    }

    lines.len() as u32
}
