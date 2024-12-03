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
        let line = line.split_once("   ");

        // Ignore lines that don't have two i32 values
        if let Some((val1, val2)) = line {
            if let (Ok(parsed1), Ok(parsed2)) = (val1.parse::<i32>(), val2.parse::<i32>()) {
                list1.push(parsed1);
                list2.push(parsed2);
            }
        }
    }

    Ok((list1, list2))
}

pub fn part_a(input: &PathBuf) -> anyhow::Result<i32> {
    let (list1, list2) = read_numbers(input)?;

    Ok(list1
        .into_iter()
        .sorted()
        .zip_eq(list2.into_iter().sorted())
        .map(|(x, y)| (x - y).abs())
        .sum())
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<i32> {
    let (list1, list2) = read_numbers(input)?;

    let mut counts = HashMap::new();
    for x in list2 {
        *counts.entry(x).or_insert(0) += 1;
    }

    Ok(list1
        .into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum())
}
