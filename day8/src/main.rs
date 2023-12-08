use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
}

fn part_one(input: &str) -> u32 {
    let mut steps = 0;
    let instructions = input.lines().next().unwrap();
    let nodes = input
        .lines()
        .skip(2)
        .filter_map(parse_line)
        .collect::<HashMap<_, _>>();

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
