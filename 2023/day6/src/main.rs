const VELOCITY: i64 = 1;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    let (times_line, distances_line) = get_lines(input);

    let times: Vec<i64> = times_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = distances_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect();

    let mut total = 1;

    for race in races {
        total *= calculate_possible_wins(race);
    }

    total
}

fn part_two(input: &str) -> i64 {
    let (times_line, distances_line) = get_lines(input);

    let time = times_line
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let distance = distances_line
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let race = Race { time, distance };

    calculate_possible_wins(race)
}

fn get_lines(input: &str) -> (&str, &str) {
    let mut lines = input.lines();
    let times_line = lines.next().unwrap();
    let distances_line = lines.next().unwrap();

    (times_line, distances_line)
}

fn calculate_possible_wins(race: Race) -> i64 {
    let mut possible_wins = 0;

    for i in 0..race.time {
        let distance = (i * VELOCITY) * (race.time - i);
        if distance > race.distance {
            possible_wins += 1;
        }
    }

    possible_wins
}
