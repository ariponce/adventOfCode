fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(parse_pattern)
        .map(summarize_reflections)
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(parse_pattern)
        .map(summarize_reflections_with_smudge)
        .sum()
}

fn parse_pattern(pattern: &str) -> Vec<Vec<u8>> {
    pattern
        .lines()
        .map(|line| line.chars().map(|char| (char == '#') as u8).collect())
        .collect()
}

fn summarize_reflections(pattern: Vec<Vec<u8>>) -> usize {
    let width = pattern[0].len();
    let height = pattern.len();

    for col in 1..width {
        let left_half = (0..col).rev();
        let right_half = col..width;
        if left_half
            .zip(right_half)
            .all(|(l, r)| get_col(&pattern, l) == get_col(&pattern, r))
        {
            return col;
        }
    }

    for row in 1..height {
        let top_half = (0..row).rev();
        let bottom_half = row..height;
        if top_half
            .zip(bottom_half)
            .all(|(t, b)| pattern[t] == pattern[b])
        {
            return row * 100;
        }
    }

    0
}

fn summarize_reflections_with_smudge(pattern: Vec<Vec<u8>>) -> usize {
    let width = pattern[0].len();
    let height = pattern.len();

    (1..width)
        .find(|&col| {
            (0..col)
                .rev()
                .zip(col..width)
                .fold(0, |diffs, (left, right)| {
                    if diffs > 1 {
                        return 2;
                    }
                    diffs + compare_cols(&pattern, left, right)
                })
                == 1
        })
        .map_or_else( // If no column reflections are found, check rows
            || {
                (1..height)
                    .find_map(|row| {
                        let diffs =
                            (0..row)
                                .rev()
                                .zip(row..height)
                                .fold(0, |diffs, (top, bottom)| {
                                    if diffs > 1 {
                                        return 2;
                                    }
                                    diffs + compare_rows(&pattern, top, bottom)
                                });
                        if diffs == 1 {
                            Some(row * 100)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0)
            },
            |col| col,
        )
}

fn get_col(pattern: &[Vec<u8>], col: usize) -> Vec<u8> {
    pattern.iter().map(|row| row[col]).collect()
}

fn compare_cols(pattern: &[Vec<u8>], t: usize, b: usize) -> usize {
    pattern
        .iter()
        .map(|row| (row[t], row[b]))
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn compare_rows(pattern: &[Vec<u8>], l: usize, r: usize) -> usize {
    pattern[l]
        .iter()
        .zip(&pattern[r])
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
