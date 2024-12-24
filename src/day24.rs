use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[allow(clippy::upper_case_acronyms)]
enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn new(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "and" => Ok(Op::AND),
            "or" => Ok(Op::OR),
            "xor" => Ok(Op::XOR),
            x => Err(anyhow!("bad op {}", x)),
        }
    }

    fn run(&self, left: bool, right: bool) -> bool {
        match self {
            Op::AND => left & right,
            Op::OR => left | right,
            Op::XOR => left ^ right,
        }
    }
}

struct Gate {
    left: String,
    right: String,
    op: Op,
    out: String,
}

impl Gate {
    fn from_line(s: &str) -> Self {
        let re = Regex::new(r"(\w+) (OR|AND|XOR) (\w+) -> (\w+)").unwrap();

        let (_, [left, op, right, out]) = re.captures_iter(s).next().unwrap().extract();

        Self {
            left: left.to_string(),
            right: right.to_string(),
            op: Op::new(op).unwrap(),
            out: out.to_string(),
        }
    }
}

fn build_state(s: &str) -> HashMap<String, bool> {
    let re = Regex::new(r"(\w+): (1|0)").unwrap();
    HashMap::from_iter(re.captures_iter(s).map(|c| {
        let (_, [name, val]) = c.extract();
        (name.to_string(), val == "1")
    }))
}

fn eval(state: &mut HashMap<String, bool>, gates: &mut VecDeque<Gate>) {
    while let Some(g) = gates.pop_front() {
        let out = state
            .get(&g.left)
            .and_then(|&l| state.get(&g.right).map(|&r| g.op.run(l, r)));
        if let Some(r) = out {
            state.insert(g.out.clone(), r);
        } else {
            gates.push_back(g);
        }
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let (init, gates) = s.split_once("\n\n").ok_or(anyhow!("bad format"))?;
    let mut state = build_state(init);
    let mut gates: VecDeque<Gate> = gates.lines().map(Gate::from_line).collect();

    eval(&mut state, &mut gates);

    let val = state
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc | ((v as usize) << i));

    Ok(val)
}

fn get_num(state: &HashMap<String, bool>, start: &str) -> usize {
    state
        .iter()
        .filter(|(k, _)| k.starts_with(start))
        .sorted()
        .enumerate()
        .fold(0, |acc, (i, (_, &v))| acc | ((v as usize) << i))
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let (init, gates) = s.split_once("\n\n").ok_or(anyhow!("bad format"))?;
    let mut state = build_state(init);
    println!("{}", gates.lines().sorted().join("\n"));
    let mut gates: VecDeque<Gate> = gates.lines().sorted().map(Gate::from_line).collect();

    eval(&mut state, &mut gates);

    let x = get_num(&state, "x");
    println!(" {x:b}");
    let y = get_num(&state, "y");
    println!(" {y:b}");
    let z = get_num(&state, "z");
    println!("{z:b}");

    println!("{:b}", x + y);

    Ok(z)
}
