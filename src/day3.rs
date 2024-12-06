use regex::Regex;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn find_mul_strings(contents: &str) -> anyhow::Result<Vec<(u16, u16)>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Ok(re
        .captures_iter(contents)
        .map(|x| {
            let (_, [a, b]) = x.extract();
            (a.parse::<u16>().unwrap(), b.parse::<u16>().unwrap())
        })
        .collect())
}

fn score_strings(contents: &str) -> anyhow::Result<u32> {
    let list = find_mul_strings(contents)?;
    Ok(list.into_iter().map(|(a, b)| a as u32 * b as u32).sum())
}

pub fn part_a(input: &Path) -> anyhow::Result<u32> {
    let contents = read_to_string(input)?;
    score_strings(&contents)
}

pub fn part_b(input: &Path) -> anyhow::Result<u32> {
    // Insert a do() at the start to make processing easier
    let contents = format!("do(){}", read_to_string(input)?);

    contents.split("don't()").try_fold(0, |score, x| {
        let (_, enabled) = x.split_once("do()").unwrap_or(("", ""));
        Ok(score + score_strings(enabled)?)
    })
}
