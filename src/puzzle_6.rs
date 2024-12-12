use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::zip,
};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_6/input";
    one(read_to_string(filepath).unwrap().lines().collect())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Block {
    Open,
    Closed,
}

fn one(file_lines: Vec<&str>) -> u32 {
    let mut start = (0, 0);
    let mut positions = HashSet::new();

    let grid: Vec<Vec<Block>> = file_lines
        .iter()
        .enumerate()
        .map(|(j, &line)| {
            line.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '.' => Block::Open,
                    '#' => Block::Closed,
                    '^' => {
                        start = (i, j);
                        Block::Open
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut x = start.0;
    let mut y = start.1;
    let mut dir = Direction::Up;

    let rows = grid.len();
    let cols = grid[0].len();

    positions.insert((x, y, dir));

    loop {
        match dir {
            Direction::Up => {
                if y == 0 {
                    break;
                }
                if grid[y - 1][x] == Block::Closed {
                    dir = Direction::Right;
                } else {
                    y -= 1;
                }
            }
            Direction::Down => {
                if y + 1 == rows {
                    break;
                }
                if grid[y + 1][x] == Block::Closed {
                    dir = Direction::Left;
                } else {
                    y += 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    break;
                }
                if grid[y][x - 1] == Block::Closed {
                    dir = Direction::Up;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if x + 1 == cols {
                    break;
                }
                if grid[y][x + 1] == Block::Closed {
                    dir = Direction::Down;
                } else {
                    x += 1;
                }
            }
        }
        positions.insert((x, y, dir));
    }

    positions.len() as u32
}

fn two(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    0
}
