use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

#[derive(Clone)]
struct State {
    pc: usize,
    instructions: Vec<usize>,
    a: usize,
    b: usize,
    c: usize,
    out: Vec<usize>,
}

impl State {
    fn from_text(input: &str) -> anyhow::Result<Self> {
        let re = Regex::new(
            r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (([0-7],?)+)",
        )?;
        let captures = re
            .captures_iter(input)
            .next()
            .ok_or(anyhow!("Coudn't parse"))?;
        let (_, [a, b, c, program, _]) = captures.extract();

        Ok(Self {
            pc: 0,
            instructions: program
                .split(",")
                .map(|d| d.parse::<usize>().map_err(Into::into))
                .collect::<anyhow::Result<Vec<usize>>>()?,
            a: a.parse()?,
            b: b.parse()?,
            c: c.parse()?,
            out: Vec::new(),
        })
    }

    fn with_a(&self, a: usize) -> Self {
        Self { a, ..self.clone() }
    }

    fn get_combo_operand(&self, operand: usize) -> anyhow::Result<usize> {
        match operand {
            0..=3 => Ok(operand),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(anyhow!("invalid combo operand")),
        }
    }

    fn div(&self, operand: usize) -> anyhow::Result<usize> {
        let op = self.get_combo_operand(operand)?;
        Ok(self.a / 2_usize.pow(op as u32))
    }

    fn adv(self, operand: usize) -> anyhow::Result<Self> {
        Ok(Self {
            a: self.div(operand)?,
            ..self
        })
    }

    fn bxl(self, operand: usize) -> anyhow::Result<Self> {
        let result = self.b ^ operand;
        Ok(Self { b: result, ..self })
    }

    fn bst(self, operand: usize) -> anyhow::Result<Self> {
        let result = self.get_combo_operand(operand)? % 8;
        Ok(Self { b: result, ..self })
    }

    fn jnz(self, operand: usize) -> anyhow::Result<Self> {
        if self.a == 0 {
            Ok(self)
        } else {
            Ok(Self {
                pc: operand,
                ..self
            })
        }
    }

    fn bxc(self, _operand: usize) -> anyhow::Result<Self> {
        let result = self.b ^ self.c;
        Ok(Self { b: result, ..self })
    }

    fn out(self, operand: usize) -> anyhow::Result<Self> {
        let to_show = self.get_combo_operand(operand)? % 8;
        let result = [self.out, vec![to_show]].concat();
        Ok(Self {
            out: result,
            ..self
        })
    }

    fn bdv(self, operand: usize) -> anyhow::Result<Self> {
        Ok(Self {
            b: self.div(operand)?,
            ..self
        })
    }

    fn cdv(self, operand: usize) -> anyhow::Result<Self> {
        Ok(Self {
            c: self.div(operand)?,
            ..self
        })
    }

    fn step(self) -> anyhow::Result<(Self, bool)> {
        if self.pc >= self.instructions.len() - 1 {
            return Ok((self, true));
        }
        let [opcode, operand] = self.instructions[self.pc..self.pc + 2] else {
            return Err(anyhow!("Not enough instructions left"));
        };
        let next_pc = Self {
            pc: self.pc + 2,
            ..self
        };
        let next_state = match opcode {
            0 => next_pc.adv(operand),
            1 => next_pc.bxl(operand),
            2 => next_pc.bst(operand),
            3 => next_pc.jnz(operand),
            4 => next_pc.bxc(operand),
            5 => next_pc.out(operand),
            6 => next_pc.bdv(operand),
            7 => next_pc.cdv(operand),
            _ => Err(anyhow!("Invalid opcode {}", opcode)),
        }?;
        Ok((next_state, false))
    }

    fn run(self) -> anyhow::Result<Vec<usize>> {
        let mut state = self;
        let mut halted = false;
        while !halted {
            (state, halted) = state.step()?;
        }
        Ok(state.out)
    }
}

pub fn part_a(input: &Path) -> anyhow::Result<String> {
    let s = read_to_string(input)?;
    let state = State::from_text(s.as_str())?;
    println!("{:?}", state.instructions);

    Ok(state.run()?.iter().join(","))
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let state = State::from_text(s.as_str())?;
    let program = state.instructions.clone();
    println!("{:?}", program);
    let mut a = 0;
    for j in 0..program.len() + 1 {
        a <<= 3;
        println!("{}", j);
        for i in 0..512 {
            let res = state.with_a(a + i).run()?;
            if res == program[program.len() - j..program.len()] {
                println!("i={}: {:?}", i, res);
                a += i;
                break;
            }
        }
    }
    Ok(a)
}
