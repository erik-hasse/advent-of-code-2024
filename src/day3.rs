use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn read_to_string(input: &PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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

pub fn part_a(input: &PathBuf) -> anyhow::Result<u32> {
    let contents = read_to_string(input)?;
    score_strings(&contents)
}

pub fn part_b(input: &PathBuf) -> anyhow::Result<u32> {
    let contents = read_to_string(input)?;
    let mut start = 0;
    let mut score = 0;
    loop {
        let end = contents[start..].find("don't()");
        if let Some(end) = end {
            let curr = &contents[start..start + end];
            score += score_strings(curr)?;
            let new_start = contents[start + end..].find("do()");
            if let Some(new_start) = new_start {
                start = start + end + new_start;
            } else {
                break;
            }
        } else {
            score += score_strings(&contents[start..])?;
            break;
        }
    }
    Ok(score)
}
