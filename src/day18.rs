use anyhow::anyhow;
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

fn reconstruct_path(came_from: HashMap<Vec2, Vec2>, end: Vec2) -> Vec<Vec2> {
    let mut path = vec![end];
    let mut current = end;
    while let Some(&prev) = came_from.get(&current) {
        path.insert(0, prev);
        current = prev;
    }
    path
}

fn a_star(
    blocks: &[(usize, usize)],
    size: Vec2,
    t: usize,
    start: Vec2,
    goal: Vec2,
) -> anyhow::Result<Option<Vec<Vec2>>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((0_isize, start)));
    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, heuristic(&start, &goal));
    let map = map_at_t(t, blocks, size);

    while let Some(Reverse((_, current))) = open_set.pop() {
        if current == goal {
            return Ok(Some(reconstruct_path(came_from, current)));
        }

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next = current + dir;
            let next_val = map
                .get(next.y as usize)
                .and_then(|l| l.get(next.x as usize));
            if next_val != Some(&'.') {
                continue;
            }
            let tentative_gscore = g_score.get(&current).map(|s| s + 1).unwrap_or(usize::MAX);
            let f = tentative_gscore as isize + heuristic(&next, &goal);
            if tentative_gscore < *g_score.get(&next).unwrap_or(&usize::MAX) {
                came_from.insert(next, current);
                g_score.insert(next, tentative_gscore);
                f_score.insert(next, f);
                let open_val = Reverse((f_score[&next], next));
                if !open_set.iter().any(|&Reverse((_, n))| n == next) {
                    open_set.push(open_val)
                }
            }
        }
    }

    Ok(None)
}

fn map_at_t(t: usize, blocks: &[(usize, usize)], size: Vec2) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; size.x as usize]; size.y as usize];
    blocks.iter().take(t).for_each(|(x, y)| map[*y][*x] = '#');
    map
}

fn parse(s: String) -> anyhow::Result<Vec<(usize, usize)>> {
    s.lines()
        .map(|l| {
            let (x, y) = l.split_once(",").ok_or(anyhow!("can't parse"))?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect()
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let size = Vec2 { x: 71, y: 71 };
    let start = Vec2 { x: 0, y: 0 };
    let goal = Vec2 { x: 70, y: 70 };

    let s = read_to_string(input)?;
    let blocks = parse(s)?;

    let path = a_star(&blocks, size, 1024, start, goal)?.unwrap();
    Ok(path.len() - 1)
}

pub fn part_b(input: &Path) -> anyhow::Result<(usize, usize)> {
    let max = 70;
    let goal = Vec2 { x: max, y: max };
    let size = Vec2 {
        x: goal.x + 1,
        y: goal.y + 1,
    };
    let start = Vec2 { x: 0, y: 0 };

    let s = read_to_string(input)?;
    let blocks = parse(s)?;

    let mut min = 0;
    let mut max = blocks.len();

    while max - min != 1 {
        let i = (max + min) / 2;
        let path = a_star(&blocks, size, i + 1, start, goal)?;
        if path.is_none() {
            max = i;
        } else {
            min = i;
        }
    }
    Ok(blocks[max])
}
