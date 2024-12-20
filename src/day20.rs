use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

const UP: Vec2 = Vec2 { x: 0, y: -1 };
const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn build_dist_map(map: &[Vec<char>], from: Vec2) -> HashMap<Vec2, usize> {
    let mut dists = HashMap::new();
    dists.insert(from, 0);
    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(curr) = queue.pop_front() {
        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next = curr + dir;
            if dists.contains_key(&next) {
                continue;
            }
            let c = map
                .get(next.y as usize)
                .and_then(|l| l.get(next.x as usize));
            if c == Some(&'#') {
                continue;
            }
            dists.insert(next, dists[&curr] + 1);
            queue.push_back(next);
        }
    }
    dists
}

fn find_val(map: &[Vec<char>], v: &char) -> Vec2 {
    let (y, line) = map.iter().find_position(|l| l.contains(v)).unwrap();

    let x = line.iter().position(|c| c == v).unwrap();
    Vec2 {
        x: x as isize,
        y: y as isize,
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let map = read_to_string(input)?;
    let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let start = find_val(&map, &'S');
    let end = find_val(&map, &'E');

    let dist_from_start = build_dist_map(&map, start);
    let dist_from_end = build_dist_map(&map, end);

    let best = dist_from_end[&start];

    let mut savings = HashMap::new();

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if ['.', 'S', 'E'].contains(c) {
                for dir in [UP, DOWN, LEFT, RIGHT] {
                    let curr = Vec2 {
                        x: x as isize,
                        y: y as isize,
                    };
                    let next = curr + dir + dir;
                    let next_char = map
                        .get(next.y as usize)
                        .and_then(|l| l.get(next.x as usize));

                    if next_char.is_none() || next_char == Some(&'#') {
                        continue;
                    }

                    let cheat_dist = dist_from_start[&curr] + 2 + dist_from_end[&next];
                    if cheat_dist < best {
                        *savings.entry(best - cheat_dist).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    Ok(savings
        .iter()
        .filter_map(|(&k, &v)| if k >= 100 { Some(v) } else { None })
        .sum())
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let map = read_to_string(input)?;
    let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let start = find_val(&map, &'S');
    let end = find_val(&map, &'E');

    let dist_from_start = build_dist_map(&map, start);
    let dist_from_end = build_dist_map(&map, end);

    let best = dist_from_end[&start];

    let mut savings = HashMap::new();

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if ['.', 'S', 'E'].contains(c) {
                let curr = Vec2 {
                    x: x as isize,
                    y: y as isize,
                };

                for i in -20_isize..=20 {
                    for j in -20_isize + i.abs()..=20 - i.abs() {
                        let next = Vec2 {
                            x: curr.x + i,
                            y: curr.y + j,
                        };
                        let next_char = map
                            .get(next.y as usize)
                            .and_then(|l| l.get(next.x as usize));
                        let change = next - curr;
                        let change_dist = change.x.unsigned_abs() + change.y.unsigned_abs();
                        if next_char.is_none() || next_char == Some(&'#') {
                            continue;
                        }

                        let cheat_dist =
                            dist_from_start[&curr] + change_dist + dist_from_end[&next];
                        if cheat_dist < best {
                            *savings.entry(best - cheat_dist).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", savings.iter().sorted());
    Ok(savings
        .iter()
        .filter_map(|(&k, &v)| if k >= 100 { Some(v) } else { None })
        .sum())
}
