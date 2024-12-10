use std::{cmp::min, cmp::max, collections::{HashMap, HashSet}, fs::read_to_string, iter::zip};
use itertools::iproduct;

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_8/input";
    one(read_to_string(filepath).unwrap().lines().collect())
}

fn one(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"[^\.]").unwrap();
    let mut antennas: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

    let cols = file_lines.len() as i32;
    let rows = file_lines[0].len() as i32;

    println!("cols {} - rows {}", cols, rows);

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

    for (&antenna, positions) in antennas.iter() {
        for (p1, p2) in iproduct!(positions, positions){
            if p1 == p2 {
                continue;
            }
            let sign = ((p1.1-p2.1)/(p1.0-p2.0)).signum();
            println!("Antenna {} for {:?} and {:?}", antenna, p1, p2);

            let dx = (p1.0-p2.0).abs();
            let dy = (p1.1-p2.1).abs();

            let anode_pos1 = (p1.0-dx, p1.1-dy);
            let anode_pos2 = (p2.0+sign*dx, p2.1+sign*dy);

            if 0 < anode_pos1.0 && anode_pos1.0 < cols && 0 < anode_pos1.1 && anode_pos1.1 < rows{
                println!("inserting {:?}", anode_pos1);
                anodes.insert(anode_pos1);
            }

            if 0 < anode_pos2.0 && anode_pos2.0 < cols && 0 < anode_pos2.1 && anode_pos2.1 < rows{
                println!("inserting {:?}", anode_pos2);
                anodes.insert(anode_pos2);
            }
        }
    }

    println!("{:?}", anodes);

    anodes.len() as u32
}

fn two(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    0
}
