use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle_10/input.txt";
    one(read_to_string(filepath).unwrap().lines())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node<'a> {
    pub height: u32,
    pub adjacencies: [Option<&'a Node<'a>>; 4], // 0 up, 1 right, 2 down, 3 left
}

impl Node<'_> {
    pub fn new(digit: u32) -> Self {
        Self {
            height: digit,
            adjacencies: [None; 4],
        }
    }

    fn climb(&self) -> u32 {
        if self.height == 9 {
            return 1;
        }

        let mut sum = 0;
        for adj in self.adjacencies {
            if let Some(adj_node) = adj {
                println!("Adjacent node: {:?}", adj_node);
                if adj_node.height == self.height + 1 {
                    sum += adj_node.climb();
                }
            }
        }
        sum
    }
}

fn one<'a>(file_lines: impl Iterator<Item = &'a str>) -> u32 {
    let mut nodes: Vec<Vec<Node>> = file_lines
        .map(|line| {
            line.chars()
                .map(|c| Node::new(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    let mut adjacency_obj = HashMap::new();
    let rows = nodes.len();
    let cols = nodes[0].len();

    let empty = Vec::new();
    for j in 0..cols {
        for i in 0..rows {
            adjacency_obj.insert((i, j), [
                nodes.get(i.wrapping_sub(1)).unwrap_or(&empty).get(j),
                nodes.get(i).unwrap_or(&empty).get(j + 1),
                nodes.get(i + 1).unwrap_or(&empty).get(j),
                nodes.get(i).unwrap_or(&empty).get(j.wrapping_sub(1)),
            ]);
        }
    }

    for j in 0..cols {
        for i in 0..rows {
            nodes.get_mut(i).unwrap().get_mut(j).unwrap().adjacencies = *adjacency_obj.get(&(i, j)).unwrap();
        }
    }

    // for (k, v) in adjacency_obj {}

    let mut sum = 0;
    // println!("nodes: {:?}", nodes);
    // for row in nodes {
    //     for node in row {
    //         if node.height == 0 {
    //             println!("node {:?} trying...", node);
    //             sum += node.climb();
    //             println!("new sum {}", sum);
    //         };
    //     }
    // }

    sum
}

fn two(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let new_lines = re
        .replace_all(file_lines.join("").as_str(), "")
        .into_owned();

    0
}
