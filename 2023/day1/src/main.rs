use std::collections::HashMap;

fn main() {
    println!("Part1 answer {}", part_one());
    println!("Part2 answer {}", part_two());
}

fn part_one() -> u32 {
    include_str!("input")
        .lines()
        .filter_map(|l| line_to_number(l))
        .sum()
}

fn part_two() -> u32 {
    include_str!("input")
        .lines()
        .map(|l| replace_str_numbers(l))
        .filter_map(|l| line_to_number(&l))
        .sum()
}

fn line_to_number(line: &str) -> Option<u32> {
    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits.next()?.to_digit(10)?;
    let last = digits
        .last()
        .unwrap_or_else(|| first.to_string().chars().next().unwrap())
        .to_digit(10)?;
    Some(first * 10 + last)
}

fn replace_str_numbers(input: &str) -> String {
    let map = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let mut result = String::from(input);
    for (word, number) in &map {
        result = result.replace(word, number);
    }

    result
}
