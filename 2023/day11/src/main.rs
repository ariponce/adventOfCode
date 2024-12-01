use std::collections::HashMap;

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Universe {
    galaxies: Vec<Point>,
    rows: HashMap<usize, u64>,
    cols: HashMap<usize, u64>,
    dimensions: (usize, usize),
}

fn get_galaxies(input: &str) -> Universe {
    let (galaxies, rows, cols) = input.lines().enumerate().fold(
        (Vec::new(), HashMap::new(), HashMap::new()),
        |(mut galaxies, mut rows, mut cols), (row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                if c == '#' {
                    galaxies.push((col, row));
                    *rows.entry(row).or_insert(0) += 1;
                    *cols.entry(col).or_insert(0) += 1;
                }
            });
            (galaxies, rows, cols)
        },
    );

    Universe {
        galaxies,
        rows,
        cols,
        dimensions: (input.lines().count(), input.lines().next().unwrap().len()),
    }
}

fn expand_space(space: &Universe, expansion: usize) -> Universe {
    let row_modifier = calc_expansion_rate(&space.rows, space.dimensions.1, expansion);
    let col_modifier = calc_expansion_rate(&space.cols, space.dimensions.0, expansion);

    Universe {
        galaxies: space
            .galaxies
            .iter()
            .map(|&(x, y)| (x + col_modifier[x], y + row_modifier[y]))
            .collect(),
        ..space.clone()
    }
}

fn calc_expansion_rate(elems: &HashMap<usize, u64>, length: usize, expansion: usize) -> Vec<usize> {
    (0..length)
        .scan(0, |modifier, i| {
            if elems.get(&i).is_none() {
                *modifier += expansion;
            }
            Some(*modifier)
        })
        .collect()
}

fn calculate_total_distance(space: &Universe) -> usize {
    space
        .galaxies
        .iter()
        .enumerate()
        .fold(0, |sum, (i, &galaxy1)| {
            sum + space.galaxies[i + 1..]
                .iter()
                .map(|&galaxy2| manhattan_distance(galaxy1, galaxy2))
                .sum::<usize>()
        })
}

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    (p1.0 as i32 - p2.0 as i32).abs() as usize + (p1.1 as i32 - p2.1 as i32).abs() as usize
}

fn part_one(input: &str) -> usize {
    let space = get_galaxies(input);
    let expanded_space = expand_space(&space, 1);

    calculate_total_distance(&expanded_space)
}

fn part_two(input: &str) -> usize {
    let space = get_galaxies(input);
    let expanded_space = expand_space(&space, 999999);

    calculate_total_distance(&expanded_space)
}

fn main() {
    let input = include_str!("input");
    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}
