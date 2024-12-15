use anyhow::anyhow;
use std::ops::Add;
use std::path::Path;
use std::{thread, time};

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

const UP: Vec2 = Vec2 { x: 0, y: -1 };
const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vec2 {
    fn from(p: usize, line_length: usize) -> Self {
        let (x, y) = (p % (line_length + 1), p / (line_length + 1));
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn get_loc(&self, line_length: usize) -> anyhow::Result<usize> {
        if self.x < 0 || self.y < 0 {
            Err(anyhow!("Can't convert a negative point"))
        } else {
            Ok(self.y as usize * (line_length + 1) + self.x as usize)
        }
    }

    fn score(&self) -> isize {
        100 * self.y + self.x
    }
}

fn step(map: &mut String, pos: &Vec2, dir: &Vec2, line_length: usize) -> anyhow::Result<()> {
    let curr = pos.get_loc(line_length)?;
    let curr_val = map.chars().nth(curr).ok_or_else(|| anyhow!("No curr"))?;
    let next_vec = *pos + *dir;
    let next = next_vec.get_loc(line_length)?;
    let next_val = map.chars().nth(next).ok_or_else(|| anyhow!("No next"))?;

    if next_val == '.' {
        map.replace_range(curr..curr + 1, ".");
        map.replace_range(next..next + 1, &curr_val.to_string())
    } else if ['O', '[', ']'].contains(&next_val) {
        step(map, &next_vec, dir, line_length)?;
        let new_next = map.chars().nth(next).unwrap();

        if new_next == '.' {
            map.replace_range(curr..curr + 1, ".");
            map.replace_range(next..next + 1, &curr_val.to_string())
        }
    }

    Ok(())
}

fn step_up_down(
    map: &mut String,
    left_pos: &Vec2,
    dir: &Vec2,
    line_length: usize,
) -> anyhow::Result<bool> {
    let curr = left_pos.get_loc(line_length)?;
    let init = map.clone();
    let l_next_vec = *left_pos + *dir;
    let l_next = l_next_vec.get_loc(line_length)?;
    let next_vals = map
        .get(l_next..l_next + 2)
        .ok_or_else(|| anyhow!("No next"))?;

    if next_vals.contains("#") {
        return Ok(false);
    }

    match next_vals {
        "[]" => step_up_down(map, &l_next_vec, dir, line_length),
        ".[" => step_up_down(map, &(l_next_vec + RIGHT), dir, line_length),
        "]." => step_up_down(map, &(l_next_vec + LEFT), dir, line_length),
        "][" => {
            let left_moved = step_up_down(map, &(l_next_vec + LEFT), dir, line_length)?;
            if left_moved {
                let right_moved = step_up_down(map, &(l_next_vec + RIGHT), dir, line_length)?;
                if right_moved {
                    Ok(true)
                } else {
                    *map = init;
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        ".." => Ok(false),
        _ => Err(anyhow!("Invalid combination {}", next_vals)),
    }?;
    let new_next = map
        .get(l_next..l_next + 2)
        .ok_or_else(|| anyhow!("No next"))?;

    if new_next == ".." {
        map.replace_range(curr..curr + 2, "..");
        map.replace_range(l_next..l_next + 2, "[]");
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_direction(c: char) -> Option<Vec2> {
    match c {
        '^' => Some(UP),
        'v' => Some(DOWN),
        '>' => Some(RIGHT),
        '<' => Some(LEFT),
        _ => None,
    }
}

fn find_robot(map: &str, line_length: usize) -> anyhow::Result<Vec2> {
    Ok(Vec2::from(
        map.find("@").ok_or_else(|| anyhow!("No robot"))?,
        line_length,
    ))
}

fn score(map: &str, line_length: usize) -> isize {
    ['O', '[']
        .iter()
        .flat_map(|c| {
            map.match_indices(*c)
                .map(|(p, _)| Vec2::from(p, line_length).score())
        })
        .sum()
}

fn double_map(map: String) -> String {
    map.replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

pub fn part_a(input: &Path) -> anyhow::Result<isize> {
    const SHOW_PROGRESS: bool = false;

    let s = read_to_string(input)?;
    let (map, directions) = s.split_once("\n\n").unwrap();
    let line_length = map.lines().next().unwrap().len();
    let mut map = map.to_string();
    println!("{}", map);

    for d in directions.chars() {
        let robot = find_robot(&map, line_length)?;
        if let Some(dir) = parse_direction(d) {
            step(&mut map, &robot, &dir, line_length)?;
            if SHOW_PROGRESS {
                thread::sleep(time::Duration::from_secs_f32(0.25));
                print!("{}[2J", 27 as char);
                println!("dir={}", d);
                println!("{}", map);
            }
        }
    }
    if !SHOW_PROGRESS {
        thread::sleep(time::Duration::from_secs_f32(0.25));
        print!("{}[2J", 27 as char);
        println!("Final");
        println!("{}", map);
    }

    Ok(score(&map, line_length))
}

pub fn part_b(input: &Path) -> anyhow::Result<isize> {
    // This whole solution is a disaster...
    let show_progress: bool = false;

    let s = read_to_string(input)?;
    let (map, directions) = s.split_once("\n\n").unwrap();
    let mut map = double_map(map.to_string());
    let line_length = map.lines().next().unwrap().len();
    println!("{}", map);

    for (i, d) in directions.chars().enumerate() {
        let robot = find_robot(&map, line_length)?;
        if let Some(dir) = parse_direction(d) {
            if (dir == UP) || (dir == DOWN) {
                let next_vec = robot + dir;
                let next = next_vec.get_loc(line_length)?;
                let next_val = map.chars().nth(next).ok_or_else(|| anyhow!("No next"))?;
                if next_val == '[' {
                    step_up_down(&mut map, &next_vec, &dir, line_length)?;
                } else if next_val == ']' {
                    step_up_down(&mut map, &(next_vec + LEFT), &dir, line_length)?;
                }
                let new_next = map.chars().nth(next).ok_or_else(|| anyhow!("No next"))?;
                if new_next == '.' {
                    step(&mut map, &robot, &dir, line_length)?;
                }
            } else {
                step(&mut map, &robot, &dir, line_length)?;
            }

            if show_progress {
                thread::sleep(time::Duration::from_secs_f32(0.25));
                print!("{}[2J", 27 as char);
                println!("{}: dir={}", i, d);
                println!("{}", map);
            }
        }
    }
    if !show_progress {
        thread::sleep(time::Duration::from_secs_f32(0.25));
        print!("{}[2J", 27 as char);
        println!("Final");
        println!("{}", map);
    }

    Ok(score(&map, line_length))
}
