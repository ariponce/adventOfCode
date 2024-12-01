fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}   

fn part_one(input: &str) -> u32 {
    input.lines()
        .filter_map(|l| l.split_once(":"))
        .map(|(_, numbers)| {
            let (winning_numbers, numbers) = numbers.split_once("|").unwrap();
            let num_set: std::collections::HashSet<_> = numbers.trim().split_whitespace().collect();
            let common = winning_numbers.trim().split_whitespace()
                .filter(|n| num_set.contains(n))
                .collect::<Vec<_>>();
            let mut points = 0;
            for _ in common {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }

            Some(points)
        })
        .sum::<Option<u32>>()
        .unwrap_or_else(|| {
            println!("No winning numbers");
            0
        })
}

fn part_two(input: &str) -> u32 {
    let mut count = 0;
    let mut card_count = vec![1u32];
    let mut matches: usize;

    for (n, line) in input.lines().enumerate() {
        let mut words = line.split_whitespace().skip(2);
        let mut winning = Vec::new();
        while let Some(x) = words.next() {
            if x == "|" {
                break;
            }
            winning.push(x);
        }
        matches = words.filter(|x| winning.contains(x)).count();
        let end = n +  matches + 1;

        if end > card_count.len() {
            card_count.resize(end, 1);
        }

        for i in n + 1..end {
            card_count[i] += card_count[n];
        }

        count += card_count[n];
    }

    count
}
