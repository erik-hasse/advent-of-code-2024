use std::collections::HashSet;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct GuardLocation {
    location: Point,
    direction: Direction,
}

impl GuardLocation {
    fn next(&self) -> Point {
        let mut location = self.location;
        match self.direction {
            Direction::Up => {
                location.y -= 1;
            }
            Direction::Right => {
                location.x += 1;
            }
            Direction::Down => {
                location.y += 1;
            }
            Direction::Left => {
                location.x -= 1;
            }
        }
        location
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    guard: Option<GuardLocation>,
    visited: HashSet<Point>,
    guard_locations: HashSet<GuardLocation>,
    looped: bool,
}

impl Grid {
    fn new(grid: String) -> Self {
        let guard = grid.replace("\n", "").find('^').unwrap();
        let grid = grid.replace("^", ".");
        let grid: Vec<Vec<char>> = grid.lines().map(|x| x.chars().collect()).collect();
        let guard = GuardLocation {
            location: Point {
                x: (guard % grid[0].len()) as isize,
                y: (guard / grid[0].len()) as isize,
            },
            direction: Direction::Up,
        };
        let mut visited = HashSet::new();
        let mut guard_locations = HashSet::new();
        visited.insert(guard.location);
        guard_locations.insert(guard);
        Self {
            grid,
            guard: Some(guard),
            visited,
            guard_locations,
            looped: false,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.y < self.grid.len() as isize
            && point.x < self.grid[0].len() as isize
    }

    fn next_guard(&mut self) {
        if let Some(mut guard) = self.guard.take() {
            let next_loc = guard.next();
            if self.contains(&next_loc) {
                if self.grid[next_loc.y as usize][next_loc.x as usize] == '#' {
                    guard.direction = match guard.direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                    self.guard = Some(guard);
                } else {
                    guard.location = next_loc;
                    self.guard = Some(guard);
                }
            } else {
                self.guard = None;
            }
        }
    }

    fn next(&mut self) {
        self.next_guard();
        self.guard.into_iter().for_each(|g| {
            if !self.guard_locations.contains(&g) {
                self.guard_locations.insert(g);
            } else {
                self.looped = true;
            }
        });
        match self.guard {
            None => (),
            Some(ref guard) => {
                self.visited.insert(guard.location);
            }
        }
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let mut grid = Grid::new(read_to_string(input)?);
    while grid.guard.is_some() {
        grid.next();
    }
    Ok(grid.visited.len())
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let input_str = read_to_string(input)?;
    let mut count = 0;
    let str_len = input_str.len();
    for i in 0..str_len {
        println!("{}/{}", i, str_len);
        if input_str.chars().nth(i) != Some('.') {
            continue;
        }
        let mut vec_string = input_str.chars().collect::<Vec<char>>();
        vec_string[i] = '#';
        let new_str: String = vec_string.iter().collect();

        let mut grid = Grid::new(new_str);
        while grid.guard.is_some() && !grid.looped {
            grid.next();
        }
        if grid.guard.is_some() {
            count += 1;
        }
    }
    Ok(count)
}
