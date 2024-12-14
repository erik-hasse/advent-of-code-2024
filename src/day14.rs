use regex::Regex;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_n(&self, v: &Point, n: isize, map_size: &Point) -> Point {
        Point {
            x: (self.x + n * v.x).rem_euclid(map_size.x),
            y: (self.y + n * v.y).rem_euclid(map_size.y),
        }
    }
}

fn parse(data: &str) -> anyhow::Result<Vec<(Point, Point)>> {
    // p=0,4 v=3,-3
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;
    Ok(re
        .captures_iter(data)
        .map(|c| {
            let (_, [x, y, vx, vy]) = c.extract();
            (
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                Point {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            )
        })
        .collect())
}

fn find_quadrant(loc: &Point, size: &Point) -> Option<usize> {
    let mid_x = size.x / 2;
    let mid_y = size.y / 2;
    let Point { x, y } = *loc;
    if x < mid_x && y < mid_y {
        return Some(0);
    } else if x > mid_x && y < mid_y {
        return Some(1);
    } else if x < mid_x && y > mid_y {
        return Some(2);
    } else if x > mid_x && y > mid_y {
        return Some(3);
    }
    None
}

fn quadrant_counts(locs: &Vec<Point>, size: &Point) -> [usize; 4] {
    let mut quads = [0, 0, 0, 0];
    for loc in locs {
        if let Some(q) = find_quadrant(loc, size) {
            quads[q] += 1
        }
    }
    quads
}

fn show(locs: &[Point], size: &Point) -> String {
    let mut m = vec![vec![0; size.x as usize]; size.y as usize];
    locs.iter().for_each(|p| m[p.y as usize][p.x as usize] += 1);

    let res: Vec<String> = m
        .iter()
        .map(|r| {
            let res: Vec<String> = r.iter().map(|x| x.to_string().replace("0", ".")).collect();
            res.join("")
        })
        .collect();
    res.join("\n")
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let map_size = Point { x: 101, y: 103 };
    let s = read_to_string(input)?;
    let points = parse(&s)?;

    let after_time: Vec<Point> = points
        .iter()
        .map(|(point, velocity)| point.move_n(velocity, 100, &map_size))
        .collect();

    let quads = quadrant_counts(&after_time, &map_size);
    println!("{:?}", quads);

    Ok(quads.iter().product())
}

pub fn part_b(input: &Path) -> anyhow::Result<()> {
    let map_size = Point { x: 101, y: 103 };
    let s = read_to_string(input)?;
    let points = parse(&s)?;

    let i = 7753;

    println!("i={}", i);
    let after_time: Vec<Point> = points
        .iter()
        .map(|(point, velocity)| point.move_n(velocity, i, &map_size))
        .collect();
    println!("{}", show(&after_time, &map_size));

    Ok(())
}
