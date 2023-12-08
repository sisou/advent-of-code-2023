use std::collections::HashMap;

use num::integer::lcm;
use regex::Regex;

fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

struct Node {
    left: String,
    right: String,
}

/// It turns out, this is a LCM (Least Common Multiple) problem. Every ghost is designed to only
/// ever come across one end node (a node ending in Z) and loop back to the same node. The number
/// of steps from the start to the first time reaching the node and then between each subsequent
/// time reaching the node is the same and stored in the ghost as the period length.
///
/// The LCM of all the period lengths is the wanted number.

#[derive(Debug, Clone)]
struct Ghost {
    node_id: String,
    step: usize,
}

impl Ghost {
    pub fn new(node_id: String) -> Self {
        Self { node_id, step: 0 }
    }

    pub fn walk(&mut self, nodes: &HashMap<String, Node>, instruction: char) -> bool {
        let node = nodes.get(&self.node_id).unwrap();
        match instruction {
            'L' => self.node_id = node.left.clone(),
            'R' => self.node_id = node.right.clone(),
            _ => unreachable!(),
        }
        self.step += 1;
        self.arrived()
    }

    pub fn arrived(&self) -> bool {
        self.node_id.ends_with('Z')
    }
}

fn run(input: &str) -> String {
    let mut lines = input.lines();

    let instructions: Vec<_> = lines.next().unwrap().chars().collect();

    let mut nodes = HashMap::new();

    let regex = Regex::new(r"^([1-9A-Z]{3}) = \(([1-9A-Z]{3}), ([1-9A-Z]{3})\)$").unwrap();

    for line in lines.skip(1) {
        let caputures = regex.captures(line).unwrap();
        let node = Node {
            left: caputures.get(2).unwrap().as_str().to_string(),
            right: caputures.get(3).unwrap().as_str().to_string(),
        };
        nodes.insert(caputures.get(1).unwrap().as_str().to_string(), node);
    }

    let mut ghosts = vec![];

    for id in nodes.keys() {
        if !id.ends_with('A') {
            continue;
        }
        ghosts.push(Ghost::new(id.to_string()));
    }

    let mut periods = vec![];

    // Find each ghost's period
    for mut ghost in ghosts {
        let mut step = 0usize;
        loop {
            if step >= instructions.len() {
                step = 0;
            }
            let instruction = instructions[step];
            let arrived = ghost.walk(&nodes, instruction);
            if arrived {
                periods.push(ghost.step);
                break;
            }
            step += 1;
        }
    }

    println!("Periods: {:#?}", periods);

    periods
        .iter()
        .fold(1usize, |acc, x| lcm(acc, *x))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, "6")
    }
}
