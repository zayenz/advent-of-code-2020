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

fn solve(input: &mut Input) -> Result<Output, Error> {
    let mut pos = 0;
    let mut trees = 0;
    for line in input {
        if line[pos % line.len()] {
            trees += 1;
        }
        pos += 3;
    }
    Ok(trees)
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
