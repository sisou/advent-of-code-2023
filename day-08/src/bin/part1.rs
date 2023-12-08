use std::collections::HashMap;

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

fn run(input: &str) -> String {
    let mut lines = input.lines();

    let instructions: Vec<_> = lines.next().unwrap().chars().collect();

    let mut nodes = HashMap::new();

    let regex = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();

    for line in lines.skip(1) {
        let caputures = regex.captures(line).unwrap();
        let node = Node {
            left: caputures.get(2).unwrap().as_str().to_string(),
            right: caputures.get(3).unwrap().as_str().to_string(),
        };
        nodes.insert(caputures.get(1).unwrap().as_str().to_string(), node);
    }

    let mut step = 0usize;
    let mut current_node_id = "AAA";
    let mut instruction_repetitions = 0;

    loop {
        if step >= instructions.len() {
            step = 0;
            instruction_repetitions += 1;
        }
        let instruction = instructions[step];
        let node = nodes.get(current_node_id).unwrap();
        match instruction {
            'L' => current_node_id = &node.left,
            'R' => current_node_id = &node.right,
            _ => unreachable!(),
        }
        step += 1;
        if current_node_id == "ZZZ" {
            break;
        }
    }

    (instruction_repetitions * step + step).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, "2");

        let result = run("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, "6")
    }
}
