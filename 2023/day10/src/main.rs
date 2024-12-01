use std::collections::VecDeque;

#[derive(Copy, Clone, Default, Debug)]
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Ground,
    Start,
}

#[derive(Debug)]
struct Node {
    tile: Tile,
    pipe: Pipe,
    visited: bool,
    distance: i32,
    in_main_loop: bool,
    enclosed_by_loop: bool,
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe::Vertical),
            '-' => Some(Pipe::Horizontal),
            'L' => Some(Pipe::TopRight),
            'J' => Some(Pipe::TopLeft),
            'F' => Some(Pipe::BottomRight),
            '7' => Some(Pipe::BottomLeft),
            '.' => Some(Pipe::Ground),
            'S' => Some(Pipe::Start),
            _ => None,
        }
    }

    fn can_go_up(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::TopLeft | Pipe::TopRight)
    }

    fn can_go_down(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::BottomLeft | Pipe::BottomRight)
    }

    fn can_go_left(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::TopLeft | Pipe::BottomLeft)
    }

    fn can_go_right(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::TopRight | Pipe::BottomRight)
    }
}

impl Node {
    /// Returns the tiles that this node is connected to, based on the pipe type.
    fn connection_tiles(&self) -> Vec<Tile> {
        match self.pipe {
            Pipe::Vertical => vec![
                Tile {
                    x: self.tile.x,
                    y: self.tile.y - 1,
                },
                Tile {
                    x: self.tile.x,
                    y: self.tile.y + 1,
                },
            ],
            Pipe::Horizontal => vec![
                Tile {
                    x: self.tile.x - 1,
                    y: self.tile.y,
                },
                Tile {
                    x: self.tile.x + 1,
                    y: self.tile.y,
                },
            ],
            Pipe::TopRight => vec![
                Tile {
                    x: self.tile.x,
                    y: self.tile.y - 1,
                },
                Tile {
                    x: self.tile.x + 1,
                    y: self.tile.y,
                },
            ],
            Pipe::TopLeft => vec![
                Tile {
                    x: self.tile.x,
                    y: self.tile.y - 1,
                },
                Tile {
                    x: self.tile.x - 1,
                    y: self.tile.y,
                },
            ],
            Pipe::BottomRight => vec![
                Tile {
                    x: self.tile.x,
                    y: self.tile.y + 1,
                },
                Tile {
                    x: self.tile.x + 1,
                    y: self.tile.y,
                },
            ],
            Pipe::BottomLeft => vec![
                Tile {
                    x: self.tile.x,
                    y: self.tile.y + 1,
                },
                Tile {
                    x: self.tile.x - 1,
                    y: self.tile.y,
                },
            ],
            Pipe::Ground | Pipe::Start => panic!("Cannot find connection tiles for ground"),
        }
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<Node>>, Tile) {
    let mut start = Tile::default();
    let mut grid: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| parse_node(c, x, y, &mut start))
                .collect()
        })
        .collect();

    replace_start(&mut grid, start);
    (grid, start)
}

fn parse_node(c: char, x: usize, y: usize, start: &mut Tile) -> Node {
    let pipe = Pipe::from_char(c).expect("Unknown pipe");
    if pipe == Pipe::Start {
        *start = Tile { x, y };
    }
    Node {
        tile: Tile { x, y },
        visited: false,
        distance: 0,
        in_main_loop: false,
        enclosed_by_loop: false,
        pipe,
    }
}

fn get_node(grid: &[Vec<Node>], tile: Tile) -> &Node {
    &grid[tile.y][tile.x]
}

fn get_node_as_mut(grid: &mut [Vec<Node>], tile: Tile) -> &mut Node {
    &mut grid[tile.y][tile.x]
}

/// Visits the given tile, marking it as visited and setting the distance.
fn visit_node(grid: &mut [Vec<Node>], tile: Tile, distance: i32) -> i32 {
    let node = &mut grid[tile.y][tile.x];
    if node.visited {
        return distance;
    }

    node.visited = true;
    node.distance = distance;
    distance
}

/// Replaces the start node with the correct pipe type.
fn replace_start(grid: &mut [Vec<Node>], start: Tile) {
    let can_go_up = match start.y.checked_sub(1) {
        Some(y) => match grid.get(y).and_then(|r| r.get(start.x)) {
            Some(node) => node.pipe.can_go_down(),
            None => false,
        },
        None => false,
    };

    let can_go_down = match start.y.checked_add(1) {
        Some(y) => match grid.get(y).and_then(|r| r.get(start.x)) {
            Some(node) => node.pipe.can_go_up(),
            None => false,
        },
        None => false,
    };

    let can_go_left = match start.x.checked_sub(1) {
        Some(x) => match grid.get(start.y).and_then(|r| r.get(x)) {
            Some(node) => node.pipe.can_go_right(),
            None => false,
        },
        None => false,
    };

    let can_go_right = match start.x.checked_add(1) {
        Some(x) => match grid.get(start.y).and_then(|r| r.get(x)) {
            Some(node) => node.pipe.can_go_left(),
            None => false,
        },
        None => false,
    };

    let pipe = match (can_go_up, can_go_right, can_go_down, can_go_left) {
        (true, true, false, false) => Pipe::TopRight,
        (true, false, false, true) => Pipe::TopLeft,
        (false, true, true, false) => Pipe::BottomRight,
        (false, false, true, true) => Pipe::BottomLeft,
        (true, false, true, false) => Pipe::Vertical,
        (false, true, false, true) => Pipe::Horizontal,
        _ => panic!("Cannot replace start"),
    };

    let start = grid
        .get_mut(start.y)
        .expect("row exists")
        .get_mut(start.x)
        .expect("column exists");
    start.pipe = pipe;
}

/// Walks the main loop of the grid from the given start, marking all nodes that are part of the main loop.
fn walk_main(grid: &mut [Vec<Node>], start: Tile) {
    let mut current = start;

    while let Some(next_tile) = find_next_tile(grid, current) {
        current = next_tile;
    }
}

/// Finds the next tile in the main loop of the grid, starting from the given tile.
fn find_next_tile(grid: &mut [Vec<Node>], current: Tile) -> Option<Tile> {
    get_node(grid, current)
        .connection_tiles()
        .into_iter()
        .find(|&tile| walk_node(grid, tile))
}

/// Marks the given tile as part of the main loop of the grid.
fn walk_node(grid: &mut [Vec<Node>], tile: Tile) -> bool {
    let node = get_node_as_mut(grid, tile);
    if !node.in_main_loop {
        node.in_main_loop = true;
        true
    } else {
        false
    }
}

// Main functions

fn part_one(input: &str) -> i32 {
    let (mut grid, start) = parse_grid(input);
    let mut visited_nodes = VecDeque::new();
    visited_nodes.push_back(start);

    let mut max_distance = 0;
    while let Some(current_tile) = visited_nodes.pop_front() {
        let current_node = get_node(&grid, current_tile);
        let distance = current_node.distance;

        let next_tiles: Vec<_> = current_node
            .connection_tiles()
            .iter()
            .filter(|&&p| !get_node(&grid, p).visited)
            .copied()
            .collect();

        for next_tile in next_tiles {
            let new_distance = visit_node(&mut grid, next_tile, distance + 1);
            max_distance = max_distance.max(new_distance);
            visited_nodes.push_back(next_tile);
        }
    }
    max_distance
}

fn part_two(input: &str) -> i32 {
    let (mut grid, start) = parse_grid(input);
    walk_main(&mut grid, start);
    grid.iter_mut()
        .map(|row| {
            let mut inside = false;
            let mut inside_count = 0;

            row.iter_mut().for_each(|node| {
                let pipe = node.pipe;

                if !node.in_main_loop && inside {
                    node.enclosed_by_loop = true;
                    inside_count += 1;
                } else if node.in_main_loop
                    && (pipe == Pipe::Vertical || pipe == Pipe::TopRight || pipe == Pipe::TopLeft)
                {
                    inside = !inside;
                }
            });

            inside_count
        })
        .sum()
}

fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}
