use std::collections::HashMap;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn parse(input: &str) -> anyhow::Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|x| x.parse().map_err(Into::into))
        .collect()
}

fn next_stones(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if stone.ilog10() % 2 == 1 {
        let parts = stone.to_string();
        let (left, right) = parts.split_at(parts.len() / 2);
        vec![left.parse().unwrap(), right.parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

fn step(stones: &[u64]) -> anyhow::Result<Vec<u64>> {
    Ok(stones
        .iter()
        .flat_map(|&stone| next_stones(stone).into_iter())
        .collect())
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let input = read_to_string(input)?;
    let mut stones = parse(&input)?;

    for i in 0..25 {
        println!("{}/25: {}", i, stones.len());
        stones = step(&stones)?;
    }

    Ok(stones.len())
}

#[allow(clippy::map_entry)]
fn count_stones_after_steps(cache: &mut HashMap<(u64, u64), u64>, stone: u64, steps: u64) -> u64 {
    if steps == 0 {
        return 1;
    }
    if !cache.contains_key(&(stone, steps)) {
        let next = next_stones(stone);
        let count = next
            .iter()
            .map(|&next_stone| count_stones_after_steps(cache, next_stone, steps - 1))
            .sum();
        cache.insert((stone, steps), count);
    }
    cache[&(stone, steps)]
}

pub fn part_b(input: &Path) -> anyhow::Result<u64> {
    let input = read_to_string(input)?;
    let stones = parse(&input)?;

    let mut cache = HashMap::new();
    Ok(stones
        .into_iter()
        .map(|stone| count_stones_after_steps(&mut cache, stone, 75))
        .sum())
}
