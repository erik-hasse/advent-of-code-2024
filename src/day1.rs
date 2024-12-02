use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

fn read_numbers(input: &PathBuf) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(input)?;
    let reader = std::io::BufReader::new(file);
    let mut list1: Vec<i32> = Vec::with_capacity(1000);
    let mut list2: Vec<i32> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        // Ignore lines that don't have two i32 values
        if let (Some(val1), Some(val2)) = (parts.next(), parts.next()) {
            if let (Ok(parsed1), Ok(parsed2)) = (val1.parse::<i32>(), val2.parse::<i32>()) {
                list1.push(parsed1);
                list2.push(parsed2);
            }
        }
    }

    Ok((list1, list2))
}

pub fn part_a(input: &PathBuf) -> anyhow::Result<i32> {
    let (mut list1, mut list2) = read_numbers(input)?;

    list1.sort();
    list2.sort();

    Ok(list1
        .into_iter()
        .zip_eq(list2.into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum())
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<i32> {
    let (list1, list2): (Vec<i32>, Vec<i32>) = read_numbers(input)?;

    let mut counts = HashMap::new();
    for x in list2 {
        *counts.entry(x).or_insert(0) += 1;
    }

    Ok(list1
        .into_iter()
        .map(|x| x * *counts.entry(x).or_insert(0))
        .sum())
}
