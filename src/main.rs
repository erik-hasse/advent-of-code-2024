mod day1;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Problem {
    Day1A,
    Day1B,
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
    }
    Ok(())
}
