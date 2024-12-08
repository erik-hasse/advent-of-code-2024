use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use std::ops::{Add, Sub};
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + Point {
            x: -other.x,
            y: -other.y,
        }
    }
}

type Antennas = HashMap<char, Vec<Point>>;

fn parse_antennas(s: String) -> Antennas {
    let mut antennas = Antennas::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    antennas
}

fn find_antinodes(a: Point, b: Point) -> [Point; 2] {
    let d = b - a;
    [b + d, a - d]
}

fn find_all_antinodes(a: Point, b: Point, size: Point) -> Vec<Point> {
    let mut antinodes = vec![a, b];
    let d = b - a;
    let increasing = repeat(d).scan(b, |p, d| {
        *p = *p + d;
        if in_bounds(*p, size) {
            Some(*p)
        } else {
            None
        }
    });
    let decreasing = repeat(d).scan(a, |p, d| {
        *p = *p - d;
        if in_bounds(*p, size) {
            Some(*p)
        } else {
            None
        }
    });
    antinodes.extend(increasing);
    antinodes.extend(decreasing);
    antinodes
}

fn in_bounds(p: Point, size: Point) -> bool {
    p.x >= 0 && p.x < size.x && p.y >= 0 && p.y < size.y
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let size = Point {
        x: s.lines().next().unwrap().len() as i32,
        y: s.lines().count() as i32,
    };
    let mut antinodes = HashSet::new();

    let antennas = parse_antennas(s);
    for points in antennas.values() {
        for (a, b) in points.iter().tuple_combinations() {
            let antenna_antinodes = find_antinodes(*a, *b);
            for antinode in &antenna_antinodes {
                if in_bounds(*antinode, size) {
                    antinodes.insert(*antinode);
                }
            }
        }
    }
    Ok(antinodes.len())
}
pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let size = Point {
        x: s.lines().next().unwrap().len() as i32,
        y: s.lines().count() as i32,
    };
    let mut antinodes = HashSet::new();

    let antennas = parse_antennas(s);
    for points in antennas.values() {
        for (a, b) in points.iter().tuple_combinations() {
            let antenna_antinodes = find_all_antinodes(*a, *b, size);
            for antinode in &antenna_antinodes {
                if in_bounds(*antinode, size) {
                    antinodes.insert(*antinode);
                }
            }
        }
    }
    Ok(antinodes.len())
}
