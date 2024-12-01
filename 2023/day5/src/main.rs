const SECTIONS: usize = 7;

fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
}

fn part_one(input: &str) -> u64 {
    let mut lines = input.lines().skip(2);
    let maps: Vec<Vec<Mapping>> = (0..SECTIONS).map(|_| process_section(&mut lines)).collect();

    let seeds: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find(|Mapping { range, .. }| range.contains(&seed))
                    .map(|Mapping { range, value }| value + seed - range.start)
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

struct Mapping {
    range: std::ops::Range<u64>,
    value: u64,
}

fn parse_line(line: &str) -> Mapping {
    let mut parts = line.splitn(3, ' ').map(|n| n.parse::<u64>().unwrap());
    let value = parts.next().unwrap();
    let start = parts.next().unwrap();
    let length = parts.next().unwrap();
    Mapping {
        range: start..start + length,
        value,
    }
}

fn process_section<'a, I>(lines: &mut I) -> Vec<Mapping>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}
