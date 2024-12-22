use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

struct Secret {
    secret: isize,
}

impl Secret {
    fn new(secret: isize) -> Self {
        Self { secret }
    }

    fn mix(&mut self, value: isize) {
        self.secret ^= value
    }

    fn prune(&mut self) {
        self.secret %= 16777216
    }

    fn mix_prune(&mut self, value: isize) {
        self.mix(value);
        self.prune()
    }
}

impl Iterator for Secret {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.secret;
        self.mix_prune(self.secret * 64);
        self.mix_prune(self.secret / 32);
        self.mix_prune(self.secret * 2048);

        Some(prev)
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<isize> {
    let s = read_to_string(input)?;
    let nums: Vec<isize> = s
        .lines()
        .map(|l| Secret::new(l.parse().unwrap()).nth(1999).unwrap())
        .collect();

    println!("{:?}", nums);
    Ok(nums.into_iter().sum())
}

type Seq = (isize, isize, isize, isize);

fn build_price_map(secret: isize) -> HashMap<Seq, isize> {
    let mut pm = HashMap::new();
    for (p1, p2, p3, p4, p5) in Secret::new(secret)
        .take(2000)
        .map(|s| s % 10)
        .tuple_windows::<(isize, isize, isize, isize, isize)>()
    {
        let k: Seq = (p2 - p1, p3 - p2, p4 - p3, p5 - p4);
        pm.entry(k).or_insert(p5);
    }
    pm
}

pub fn part_b(input: &Path) -> anyhow::Result<isize> {
    let s = read_to_string(input)?;

    let mut prices = HashMap::new();

    for l in s.lines() {
        let pm = build_price_map(l.parse()?);
        pm.iter()
            .for_each(|(k, v)| *prices.entry(*k).or_insert(0) += v);
    }
    let (seq, price) = prices.iter().max_by_key(|&(_, v)| v).unwrap();
    println!("{} at {:?}", price, seq);
    Ok(*price)
}
