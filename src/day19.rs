use std::collections::HashMap;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn find_arrangement(goal: &[char], options: &Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    if goal.is_empty() {
        return Some(vec![]);
    }
    println!("Goal {:?}", goal);
    for t in options {
        if goal.starts_with(t.as_slice()) {
            let rest = goal[t.len()..].to_vec();
            if let Some(mut v) = find_arrangement(&rest, options) {
                v.insert(0, t.to_vec());
                return Some(v);
            }
        }
    }
    None
}

fn find_arrangements(
    cache: &mut HashMap<Vec<char>, usize>,
    goal: &[char],
    options: &Vec<Vec<char>>,
) -> usize {
    if goal.is_empty() {
        return 1;
    }
    let goal_vec = goal.to_vec();
    if cache.contains_key(&goal_vec) {
        return cache[&goal_vec];
    }
    let mut result = 0;
    for t in options {
        if goal.starts_with(t.as_slice()) {
            let rest = goal[t.len()..].to_vec();
            result += find_arrangements(cache, &rest, options);
        }
    }
    cache.insert(goal_vec, result);
    result
}

fn parse(s: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (options, goals) = s.split_once("\n\n").unwrap();
    (
        options.split(", ").map(|o| o.chars().collect()).collect(),
        goals.lines().map(|l| l.chars().collect()).collect(),
    )
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let (options, goals) = parse(&s);
    let mut valid = 0;
    for t in goals {
        if let Some(g) = find_arrangement(&t, &options) {
            println!("{:?} = {:?}", t, g);
            valid += 1
        }
    }

    Ok(valid)
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let (options, goals) = parse(&s);
    let mut approaches = 0;
    let mut cache = HashMap::new();
    for t in goals {
        approaches += find_arrangements(&mut cache, &t, &options);
    }

    Ok(approaches)
}
