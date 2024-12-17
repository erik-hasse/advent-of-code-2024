use anyhow::anyhow;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn get_vec(&self) -> Vec2 {
        match self {
            Dir::North => UP,
            Dir::East => RIGHT,
            Dir::South => DOWN,
            Dir::West => LEFT,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

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

fn heuristic(a: &Vec2, b: &Vec2) -> isize {
    let diff = *b - *a;
    diff.x + diff.y
}

fn score_step(current: Vec2, dir: Dir, next: Vec2, next_dir: Dir) -> usize {
    if dir != next_dir {
        1000
    } else if current != next {
        1
    } else {
        0
    }
}

fn reconstruct_path(
    came_from: HashMap<(Vec2, Dir), (Vec2, Dir)>,
    end: Vec2,
    end_dir: Dir,
) -> Steps {
    let mut path = vec![(end, end_dir)];
    let mut current = end;
    let mut current_dir = end_dir;
    while let Some(&(prev, prev_dir)) = came_from.get(&(current, current_dir)) {
        path.insert(0, (prev, prev_dir));
        current = prev;
        current_dir = prev_dir
    }
    path
}

fn find_val(map: &[Vec<char>], v: &char) -> Vec2 {
    let (y, line) = map.iter().find_position(|l| l.contains(v)).unwrap();

    let x = line.iter().position(|c| c == v).unwrap();
    Vec2 {
        x: x as isize,
        y: y as isize,
    }
}

type Steps = Vec<(Vec2, Dir)>;
type ScoreMap<T> = HashMap<(Vec2, Dir), T>;

fn a_star(
    map: &[Vec<char>],
    start: Vec2,
    start_dir: Dir,
    goal: Vec2,
) -> anyhow::Result<(Steps, ScoreMap<usize>)> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((0_isize, start, start_dir)));
    let mut came_from = HashMap::new();

    let mut g_score: ScoreMap<usize> = HashMap::new();
    g_score.insert((start, start_dir), 0);

    let mut f_score = HashMap::new();
    f_score.insert((start, start_dir), heuristic(&start, &goal));

    while let Some(Reverse((_, current, dir))) = open_set.pop() {
        if current == goal {
            return Ok((reconstruct_path(came_from, current, dir), g_score));
        }

        let mut next_states = vec![(current, dir.turn_right()), (current, dir.turn_left())];

        let next = current + dir.get_vec();
        if map[next.y as usize][next.x as usize] != '#' {
            next_states.push((next, dir))
        }

        for (next, next_dir) in next_states {
            let tentative_gscore = g_score
                .get(&(current, dir))
                .map(|s| s + score_step(current, dir, next, next_dir))
                .unwrap_or(usize::MAX);
            let f = tentative_gscore as isize + heuristic(&next, &goal);
            if tentative_gscore < *g_score.get(&(next, next_dir)).unwrap_or(&usize::MAX) {
                came_from.insert((next, next_dir), (current, dir));
                g_score.insert((next, next_dir), tentative_gscore);
                f_score.insert((next, next_dir), f);
                let open_val = Reverse((f_score[&(next, next_dir)], next, next_dir));
                if !open_set
                    .iter()
                    .any(|&Reverse((_, n, d))| n == next && d == next_dir)
                {
                    open_set.push(open_val)
                }
            }
        }
    }

    Err(anyhow!("No path found"))
}

fn score_path(path: &Steps) -> usize {
    path.iter()
        .tuple_windows()
        .map(|((p, p_d), (n, n_d))| score_step(*p, *p_d, *n, *n_d))
        .sum::<usize>()
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let map = read_to_string(input)?;
    let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let start = find_val(&map, &'S');
    let end = find_val(&map, &'E');
    let (best_path, _) = a_star(&map, start, Dir::East, end)?;
    Ok(score_path(&best_path))
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let map = read_to_string(input)?;
    let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let start = find_val(&map, &'S');
    let end = find_val(&map, &'E');
    let (best_path, from_start) = a_star(&map, start, Dir::East, end)?;
    let best_score = score_path(&best_path);

    let (_, end_dir) = best_path.last().unwrap();
    let (_, from_end) = a_star(&map, end, end_dir.reverse(), start)?;
    let mut spots = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char != '#' {
                println!("testing {},{}", x, y);
                let curr = Vec2 {
                    x: x as isize,
                    y: y as isize,
                };
                let options = from_start
                    .iter()
                    .filter_map(|(&(p, d), v)| if p == curr { Some((v, d)) } else { None })
                    .min();
                let Some((&d_from_start, end_dir)) = options else {
                    continue;
                };
                let d_from_end = from_end.get(&(curr, end_dir.reverse()));

                if let Some(d) = d_from_end {
                    if d_from_start + d == best_score {
                        spots += 1
                    }
                }
            }
        }
    }
    Ok(spots)
}
