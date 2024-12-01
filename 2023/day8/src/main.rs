use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    let instructions = input.lines().next().unwrap();
    let nodes = input
        .lines()
        .skip(2)
        .filter_map(parse_line)
        .collect::<HashMap<_, _>>();

    println!("Part one: {}", part_one(instructions, nodes.clone()));
    println!("Part two: {}", part_two(instructions, nodes.clone()));
}

fn part_one(instructions: &str, nodes: HashMap<String, (String, String)>) -> u32 {
    let mut steps = 0;
    let mut value: String = "AAA".to_string();

    while value != "ZZZ" {
        for direction in instructions.chars() {
            if let Some((left, right)) = nodes.get(&value) {
                value = match direction {
                    'L' => left.to_owned(),
                    'R' => right.to_owned(),
                    _ => {
                        println!("Unknown direction: {}", direction);
                        continue;
                    }
                };
                steps += 1;
            } else {
                println!("Unknown value: {}", value);
                break;
            }
        }
    }

    steps
}

fn part_two(instructions: &str, nodes: HashMap<String, (String, String)>) -> usize {
    let start_nodes: Vec<&String> = nodes.keys().filter(|k| k.ends_with('A')).collect();

    let steps: Vec<usize> = start_nodes
        .iter()
        .map(|&start_node| {
            let mut node = start_node.clone();
            let mut i = 0;
            while !node.ends_with('Z') {
                for direction in instructions.chars() {
                    if let Some((left, right)) = nodes.get(&node) {
                        node = match direction {
                            'L' => left.to_string(),
                            'R' => right.to_string(),
                            _ => continue,
                        };
                        i += 1;
                    } else {
                        println!("Unknown node: {}", node);
                        break;
                    }
                }
            }
            i
        })
        .collect();

    steps.iter().fold(1, |acc, &count| lcm(acc, count))
}

fn parse_line(line: &str) -> Option<(String, (String, String))> {
    let parts: Vec<&str> = line.split('=').map(str::trim).collect();
    if parts.len() == 2 {
        let key = parts[0].to_string();
        let values: Vec<&str> = parts[1]
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(str::trim)
            .collect();

        if values.len() == 2 {
            Some((key, (values[0].to_string(), values[1].to_string())))
        } else {
            None
        }
    } else {
        None
    }
}
