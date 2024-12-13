extern crate nalgebra as na;

use nalgebra::{Matrix2, Vector2};
use regex::Regex;
use std::path::Path;

const PRESS_A_COST: usize = 3;
const PRESS_B_COST: usize = 1;
const FLOAT_TOL: f64 = 0.000_1;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

fn parse_problems(data: &str) -> anyhow::Result<Vec<(Matrix2<f64>, Vector2<f64>)>> {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )?;
    Ok(re
        .captures_iter(data)
        .map(|c| {
            let (_, [x1, y1, x2, y2, xs, ys]) = c.extract();
            let matrix: Matrix2<f64> = Matrix2::new(
                x1.parse().unwrap(),
                x2.parse().unwrap(),
                y1.parse().unwrap(),
                y2.parse().unwrap(),
            );
            let vec: Vector2<f64> = Vector2::new(xs.parse().unwrap(), ys.parse().unwrap());
            (matrix, vec)
        })
        .collect())
}

fn close_to_posint(x: f64) -> Option<usize> {
    if x < 0.0 {
        return None;
    }
    let xr = x.round();
    if (x - xr).abs() <= FLOAT_TOL {
        Some(xr as usize)
    } else {
        None
    }
}

fn solve(matrix: Matrix2<f64>, vec: Vector2<f64>) -> Option<usize> {
    println!("M={:?}, v={:?}", matrix, vec);
    let sol = matrix.lu().solve(&vec);
    sol.and_then(|s| {
        println!("s={:?}", s);
        let maybe_int_sols: Option<Vec<usize>> = s
            .transpose()
            .into_iter()
            .map(|x| close_to_posint(*x))
            .collect();
        println!("sol={:?}", maybe_int_sols);
        maybe_int_sols.map(|s| s[0] * PRESS_A_COST + s[1] * PRESS_B_COST)
    })
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let probs = parse_problems(s.as_str())?;
    let mut cost = 0;

    for (m, v) in probs {
        if let Some(p_cost) = solve(m, v) {
            cost += p_cost
        }
    }

    Ok(cost)
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let probs = parse_problems(s.as_str())?;
    let mut cost = 0;
    let error = Vector2::new(10000000000000.0, 10000000000000.0);

    for (m, v) in probs {
        if let Some(p_cost) = solve(m, v + error) {
            cost += p_cost
        }
    }

    Ok(cost)
}
