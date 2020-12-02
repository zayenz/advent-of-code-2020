#![allow(dead_code, unused_imports)]

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

struct Policy {
    min: i32,
    max: i32,
    letter: char,
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count() as i32;
        self.min <= count && count <= self.max
    }
}

impl FromStr for Policy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(|ch| ch == '-' || ch == ' ').collect_vec();
        let policy = Policy {
            min: parts[0].parse()?,
            max: parts[1].parse()?,
            letter: parts[2]
                .chars()
                .next()
                .ok_or_else(|| format_err!("No letter specified"))?,
        };
        Ok(policy)
    }
}

type Input = Vec<(Policy, String)>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let parts = line.split(": ").collect_vec();
        result.push((parts[0].parse()?, parts[1].to_string()));
    }

    Ok(result)
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let valid = input
        .iter()
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    Ok(valid)
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
