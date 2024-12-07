use itertools::Itertools;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

struct Equation {
    target: u64,
    components: Vec<u64>,
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Equation {
    fn new(s: &str) -> Self {
        let (target, parts) = s.split_once(":").unwrap();
        let target = target.parse().unwrap();
        let components = parts
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Self { target, components }
    }

    fn test_op_seq(&self, seq: &[Op]) -> bool {
        self.target
            == self.components[1..].iter().zip_eq(seq.iter()).fold(
                self.components[0],
                |acc, (a, op)| match op {
                    Op::Add => acc + a,
                    Op::Mul => acc * a,
                    Op::Concat => format!("{}{}", acc, a).parse().unwrap(),
                },
            )
    }

    fn test_all_op_seqs(&self, ops: &[Op]) -> bool {
        let size = self.components.len() - 1;
        let combinations = (0..size)
            .map(|_| ops.iter().cloned())
            .multi_cartesian_product();
        for seq in combinations {
            if self.test_op_seq(&seq) {
                println!(
                    "{}: {} works with {:?}",
                    self.target,
                    self.components.iter().join(" "),
                    seq
                );
                return true;
            }
        }
        false
    }
}

fn solve(equations: &[Equation], ops: &[Op]) -> u64 {
    let mut count = 0;
    let num_eqs = equations.len();

    for (i, eq) in equations.iter().enumerate() {
        println!("{} / {}", i, num_eqs);
        if eq.test_all_op_seqs(ops) {
            count += eq.target;
        }
    }
    count
}

pub fn part_a(input: &Path) -> anyhow::Result<u64> {
    let input = read_to_string(input)?;
    let equations: Vec<Equation> = input.lines().map(Equation::new).collect();
    let ops = [Op::Add, Op::Mul];

    Ok(solve(&equations, &ops))
}

pub fn part_b(input: &Path) -> anyhow::Result<u64> {
    let input = read_to_string(input)?;
    let equations: Vec<Equation> = input.lines().map(Equation::new).collect();
    let ops = [Op::Add, Op::Mul, Op::Concat];

    Ok(solve(&equations, &ops))
}
