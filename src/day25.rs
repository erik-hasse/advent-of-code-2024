use itertools::Itertools;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn count_col(v: &Vec<Vec<char>>, i: usize) -> usize {
    v.iter().map(|l| l[i]).filter(|c| c == &'#').count()
}

fn parse_block(s: &str) -> [usize; 5] {
    let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    [
        count_col(&grid, 0) - 1,
        count_col(&grid, 1) - 1,
        count_col(&grid, 2) - 1,
        count_col(&grid, 3) - 1,
        count_col(&grid, 4) - 1,
    ]
}

fn parse(s: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for b in s.split("\n\n") {
        let vals = parse_block(b);
        if b.starts_with("#") {
            locks.push(vals);
        } else {
            keys.push(vals);
        }
    }
    (keys, locks)
}

fn check(key: &[usize; 5], lock: &[usize; 5]) -> bool {
    let mut comp = key.into_iter().zip_eq(lock).map(|(k, l)| k + l);
    comp.all(|x| x <= 5)
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let (keys, locks) = parse(&s);
    println!("keys {keys:?}");
    println!("locks {locks:?}");
    let mut count = 0;
    for k in keys {
        for l in &locks {
            if check(&k, &l) {
                count += 1
            }
        }
    }
    Ok(count)
}
