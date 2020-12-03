#![allow(dead_code, unused_imports, clippy::ptr_arg)]

use failure::bail;
use failure::format_err;
use failure::Error;
use itertools::Itertools;
use rayon::prelude::*;
use strum_macros::EnumString;

use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

type Input = Vec<Vec<bool>>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            result.push(line.chars().map(|ch| ch == '#').collect_vec());
        }
    }

    Ok(result)
}

fn compute_trees(input: &Input, step_right: usize, step_line: usize) -> usize {
    let mut pos = 0;
    let mut trees = 0;
    for line in (0..input.len()).step_by(step_line) {
        if input[line][pos % input[line].len()] {
            trees += 1;
        }
        pos += step_right;
    }
    trees
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(step_right, step_line)| compute_trees(input, *step_right, *step_line))
        .collect_vec();
    dbg!(&trees);
    let result = trees.iter().product();
    Ok(result)
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    let output = solve(&mut input)?;

    println!("{}", output);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            eprintln!("Error while solving problem: {}", error);
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}
