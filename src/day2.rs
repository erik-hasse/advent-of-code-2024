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

fn validate_report(report: &Vec<i8>) -> bool {
    let diffs: Vec<i8> = report
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect();
    (diffs.iter().all(|&x| x < 0) || diffs.iter().all(|&x| x > 0))
        && (diffs.iter().map(|&x| x.abs()).max().unwrap_or(0) <= MAX_DIFF)
}

fn validate_dampend_report(report: &Vec<i8>) -> bool {
    (0..report.len())
        .map(|i| [&report[..i], &report[i + 1..]].concat())
        .any(|x| validate_report(&x))
}

pub fn part_a(input: &PathBuf) -> anyhow::Result<i32> {
    let list = read_numbers(input)?;
    Ok(list.into_iter().filter(validate_report).count() as i32)
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<i32> {
    let list = read_numbers(input)?;
    Ok(list.into_iter().filter(validate_dampend_report).count() as i32)
}
