use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

const MAX_DIFF: i8 = 3;

fn read_numbers(input: &PathBuf) -> anyhow::Result<Vec<Vec<i8>>> {
    let file = File::open(input)?;
    let reader = std::io::BufReader::new(file);
    let mut list: Vec<Vec<i8>> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line?;
        list.push(
            line.split_whitespace()
                .map(|x| x.parse::<i8>().unwrap())
                .collect(),
        );
    }
    Ok(list)
}

fn validate_report(report: &[i8]) -> bool {
    let is_ascending = report.is_sorted_by(|a, b| a < b);
    let is_descending = report.is_sorted_by(|a, b| a > b);
    let in_bounds = report
        .iter()
        .tuple_windows()
        .all(|(a, b)| (a - b).abs() <= MAX_DIFF);
    (is_ascending || is_descending) && in_bounds
}

fn validate_dampend_report(report: &[i8]) -> bool {
    (0..report.len()).any(|i| {
        let (left, right) = report.split_at(i);
        validate_report(&[left, &right[1..]].concat())
    })
}

pub fn part_a(input: &PathBuf) -> anyhow::Result<i32> {
    let list = read_numbers(input)?;
    Ok(list.into_iter().filter(|x| validate_report(x)).count() as i32)
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<i32> {
    let list = read_numbers(input)?;
    Ok(list
        .into_iter()
        .filter(|x| validate_dampend_report(x))
        .count() as i32)
}
