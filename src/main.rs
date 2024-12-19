mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day9b;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Problem {
    Day1A,
    Day1B,
    Day2A,
    Day2B,
    Day3A,
    Day3B,
    Day4A,
    Day4B,
    Day5A,
    Day5B,
    Day6A,
    Day6B,
    Day7A,
    Day7B,
    Day8A,
    Day8B,
    Day9A,
    Day9B,
    Day10A,
    Day10B,
    Day11A,
    Day11B,
    Day12A,
    Day12B,
    Day13A,
    Day13B,
    Day14A,
    Day14B,
    Day15A,
    Day15B,
    Day16A,
    Day16B,
    Day17A,
    Day17B,
    Day18A,
    Day18B,
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    problem: Problem,

    input: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.problem {
        Problem::Day1A => {
            println!("{}", day1::part_a(&args.input)?);
        }
        Problem::Day1B => {
            println!("{}", day1::part_b(&args.input)?);
        }
        Problem::Day2A => {
            println!("{}", day2::part_a(&args.input)?);
        }
        Problem::Day2B => {
            println!("{}", day2::part_b(&args.input)?);
        }
        Problem::Day3A => {
            println!("{:?}", day3::part_a(&args.input)?);
        }
        Problem::Day3B => {
            println!("{:?}", day3::part_b(&args.input)?);
        }
        Problem::Day4A => {
            println!("{:?}", day4::part_a(&args.input)?);
        }
        Problem::Day4B => {
            println!("{:?}", day4::part_b(&args.input)?);
        }
        Problem::Day5A => {
            println!("{:?}", day5::part_a(&args.input)?);
        }
        Problem::Day5B => {
            println!("{:?}", day5::part_b(&args.input)?);
        }
        Problem::Day6A => {
            println!("{:?}", day6::part_a(&args.input)?);
        }
        Problem::Day6B => {
            println!("{:?}", day6::part_b(&args.input)?);
        }
        Problem::Day7A => {
            println!("{:?}", day7::part_a(&args.input)?);
        }
        Problem::Day7B => {
            println!("{:?}", day7::part_b(&args.input)?);
        }
        Problem::Day8A => {
            println!("{:?}", day8::part_a(&args.input)?);
        }
        Problem::Day8B => {
            println!("{:?}", day8::part_b(&args.input)?);
        }
        Problem::Day9A => {
            println!("{:?}", day9::part_a(&args.input)?);
        }
        Problem::Day9B => {
            println!("{:?}", day9b::part_b(&args.input)?);
        }
        Problem::Day10A => {
            println!("{:?}", day10::part_a(&args.input)?);
        }
        Problem::Day10B => {
            println!("{:?}", day10::part_b(&args.input)?);
        }
        Problem::Day11A => {
            println!("{:?}", day11::part_a(&args.input)?);
        }
        Problem::Day11B => {
            println!("{:?}", day11::part_b(&args.input)?);
        }
        Problem::Day12A => {
            println!("{:?}", day12::part_a(&args.input)?);
        }
        Problem::Day12B => {
            println!("{:?}", day12::part_b(&args.input)?);
        }
        Problem::Day13A => {
            println!("{:?}", day13::part_a(&args.input)?);
        }
        Problem::Day13B => {
            println!("{:?}", day13::part_b(&args.input)?);
        }
        Problem::Day14A => {
            println!("{:?}", day14::part_a(&args.input)?);
        }
        Problem::Day14B => {
            println!("{:?}", day14::part_b(&args.input)?);
        }
        Problem::Day15A => {
            println!("{:?}", day15::part_a(&args.input)?);
        }
        Problem::Day15B => {
            println!("{:?}", day15::part_b(&args.input)?);
        }
        Problem::Day16A => {
            println!("{:?}", day16::part_a(&args.input)?);
        }
        Problem::Day16B => {
            println!("{:?}", day16::part_b(&args.input)?);
        }
        Problem::Day17A => {
            println!("{:?}", day17::part_a(&args.input)?);
        }
        Problem::Day17B => {
            println!("{:?}", day17::part_b(&args.input)?);
        }
        Problem::Day18A => {
            println!("{:?}", day18::part_a(&args.input)?);
        }
        Problem::Day18B => {
            println!("{:?}", day18::part_b(&args.input)?);
        }
    }
    Ok(())
}
