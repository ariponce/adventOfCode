fn main() {
    let input = include_str!("input");
    let sets = parse_input(input);
    println!("Part one: {}", part_one(&sets));
    println!("Part two: {}", part_two(&sets));
}

fn part_one(sets: &[Vec<i32>]) -> i32 {
    sets.iter().map(|vec| predict_last(vec)).sum()
}

fn part_two(sets: &[Vec<i32>]) -> i32 {
    sets.iter().map(|vec| predict_first(vec)).sum()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn predict_last(vec: &[i32]) -> i32 {
    let diffs = vec.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    let last = vec.last().unwrap();
    if diffs.iter().all(|&x| x == 0) {
        return *last;
    }

    last + predict_last(&diffs)
}

fn predict_first(vec: &[i32]) -> i32 {
    let diffs = vec.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    let first = vec[0];
    if diffs.iter().all(|&x| x == 0) {
        return first;
    }

    first - predict_first(&diffs)
}
