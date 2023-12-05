use regex::Regex;

struct GridSize {
    rows: i32,
    cols: i32,
}

struct Position {
    x: i32,
    y: i32,
}

impl GridSize {
    fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().chars().count() as i32;
        let rows = input.lines().count() as i32;
        GridSize { rows, cols }
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.rows && position.y >= 0 && position.y < self.cols
    }
}

fn main() {
    let input = include_str!("input");
    let grid_size = GridSize::new(input);
    println!("Part one: {}", part_one(input, &grid_size));
    println!("Part two: {}", part_two(input, &grid_size));
}

fn part_one(input: &str, grid_size: &GridSize) -> u32 {
    let numbers = get_numbers(input);
    let symbols = get_symbols(input);

    let mut parts = Vec::new();
    for (number, start_position) in &numbers {
        let end_position = Position {
            x: start_position.x,
            y: start_position.y + number.len() as i32 - 1,
        };
        for (_, symbol_position) in &symbols {
            let neighbours = get_neighbours(symbol_position)
                .into_iter()
                .filter(|pos| grid_size.is_within_bounds(pos))
                .collect::<Vec<_>>();
            for neighbour in &neighbours {
                if is_in_line_with(neighbour, start_position, &end_position) {
                    parts.push(number.parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }

    parts.iter().sum()
}

fn part_two(input: &str, grid_size: &GridSize) -> u32 {
    let numbers = get_numbers(input);
    let gears = get_gears(input);
    let mut ratios = Vec::new();

    for (_, gear_position) in &gears {
        let neighbours = get_neighbours(gear_position)
            .into_iter()
            .filter(|pos| grid_size.is_within_bounds(pos))
            .collect::<Vec<_>>();
        let mut adjacent_numbers = Vec::new();
        for (number, start_position) in &numbers {
            let end_position = Position {
                x: start_position.x,
                y: start_position.y + number.len() as i32 - 1,
            };
            for neighbour in &neighbours {
                if is_in_line_with(neighbour, start_position, &end_position) {
                    adjacent_numbers.push(number.parse::<u32>().unwrap());
                    break;
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            ratios.push(adjacent_numbers.iter().product());
        }
    }
    ratios.iter().sum()
}

fn get_neighbours(position: &Position) -> Vec<Position> {
    vec![
        Position {
            x: position.x - 1,
            y: position.y - 1,
        },
        Position {
            x: position.x - 1,
            y: position.y,
        },
        Position {
            x: position.x - 1,
            y: position.y + 1,
        },
        Position {
            x: position.x,
            y: position.y - 1,
        },
        Position {
            x: position.x,
            y: position.y + 1,
        },
        Position {
            x: position.x + 1,
            y: position.y - 1,
        },
        Position {
            x: position.x + 1,
            y: position.y,
        },
        Position {
            x: position.x + 1,
            y: position.y + 1,
        },
    ]
}

fn is_in_line_with(position: &Position, start: &Position, end: &Position) -> bool {
    position.x == start.x && position.y >= start.y && position.y <= end.y
}

fn get_numbers(input: &str) -> Vec<(&str, Position)> {
    let mut numbers = Vec::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    for (i, line) in input.lines().enumerate() {
        for regex_match in num_regex.find_iter(line) {
            numbers.push((
                regex_match.as_str(),
                Position {
                    x: i as i32,
                    y: regex_match.start() as i32,
                },
            ));
        }
    }
    numbers
}

fn get_symbols(input: &str) -> Vec<(char, Position)> {
    let mut symbols = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push((
                    c,
                    Position {
                        x: i as i32,
                        y: j as i32,
                    },
                ));
            }
        }
    }
    symbols
}

fn get_gears(input: &str) -> Vec<(char, Position)> {
    let mut gears = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push((
                    c,
                    Position {
                        x: i as i32,
                        y: j as i32,
                    },
                ));
            }
        }
    }
    gears
}
