use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter::once;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn build_shortest_map(keypad: &str) -> HashMap<(char, char), Vec<char>> {
    let mut shortest_paths = HashMap::new();
    let blank_row = (keypad.find(" ").unwrap() / 3) as isize;

    for (i, c1) in keypad.chars().enumerate() {
        let i = i as isize;
        for (j, c2) in keypad.chars().enumerate() {
            let j = j as isize;
            let (i_q, i_r) = (i / 3, i % 3);
            let (j_q, j_r) = (j / 3, j % 3);

            let up_down = j_q - i_q;
            let ud_char = if up_down >= 0 { 'v' } else { '^' };

            let left_right = j_r - i_r;
            let num_right = max(left_right, 0) as usize;
            let num_left = min(left_right, 0).unsigned_abs();

            let need_ud_first_from =
                (i_q == blank_row && j_r == 0 && num_left != 0 && up_down != 0) as usize;
            let need_ud_last_to =
                (j_q == blank_row && i_r == 0 && num_right != 0 && up_down != 0) as usize;

            let shortest = [
                vec!['<'; num_left * (1 - need_ud_first_from)],
                vec!['>'; num_right * need_ud_last_to],
                vec![ud_char; up_down.unsigned_abs()],
                vec!['>'; num_right * (1 - need_ud_last_to)],
                vec!['<'; num_left * need_ud_first_from],
                vec!['A'],
            ]
            .concat();
            println!("Shortest {} to {}: {:?}", c1, c2, shortest);
            shortest_paths.insert((c1, c2), shortest);
        }
    }

    shortest_paths
}

fn build_seq(shortest_map: &HashMap<(char, char), Vec<char>>, seq: &str) -> String {
    let cs = once('A')
        .chain(seq.chars())
        .tuple_windows()
        .map(|(c1, c2)| shortest_map[&(c1, c2)].clone())
        .concat();

    cs.iter().join("")
}

fn cost_map(
    shortest_map: &HashMap<(char, char), Vec<char>>,
    prev: &HashMap<(char, char), usize>,
) -> HashMap<(char, char), usize> {
    let mut res = HashMap::new();
    for (&c, steps) in shortest_map.iter() {
        res.insert(
            c,
            once(&'A')
                .chain(steps.iter())
                .tuple_windows()
                .map(|(&x1, &x2)| prev.get(&(x1, x2)).unwrap_or(&1))
                .sum(),
        );
    }
    res
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let codes = read_to_string(input)?;
    let numpad = "789456123 0A";
    let numpad_shortest = build_shortest_map(numpad);
    let dirpad = " ^A<v>";
    let dirpad_shortest = build_shortest_map(dirpad);
    let mut complexity = 0;
    for c in codes.lines() {
        let mut seq = build_seq(&numpad_shortest, c);
        for _ in 0..2 {
            seq = build_seq(&dirpad_shortest, &seq)
        }
        let num: usize = c[..3].parse()?;
        println! {"{} * {}", seq.chars().count(), num}
        complexity += num * seq.chars().count();
    }
    Ok(complexity)
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let codes = read_to_string(input)?;
    let numpad = "789456123 0A";
    let numpad_shortest = build_shortest_map(numpad);
    println!("{:?}", numpad_shortest);
    let dirpad = " ^A<v>";
    let dirpad_shortest = build_shortest_map(dirpad);
    println!("{:?}", dirpad_shortest);
    let mut complexity = 0;
    for c in codes.lines() {
        let mut costs = HashMap::new();
        for _ in 0..25 {
            costs = cost_map(&dirpad_shortest, &costs)
        }

        let mut cost = 0;
        let seq = build_seq(&numpad_shortest, c);
        for (c1, c2) in once('A').chain(seq.chars()).tuple_windows() {
            cost += costs[&(c1, c2)]
        }
        let num: usize = c[..3].parse()?;
        println! {"{} * {}", cost, num}
        complexity += num * cost;
    }
    Ok(complexity)
}
