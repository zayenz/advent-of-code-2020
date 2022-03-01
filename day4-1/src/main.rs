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

type Input = Vec<String>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.trim().to_string());
    }

    Ok(result)
}

fn solve(input: &Input) -> Result<Output, Error> {
    let passport_data: Vec<Vec<&String>> = input
        .into_iter()
        .group_by(|v| v.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| group.collect_vec())
        .collect_vec();

    let passports = passport_data
        .iter()
        .map(|lines| {
            lines
                .iter()
                .flat_map(|line| {
                    line.split_ascii_whitespace().map(|kv| {
                        kv.split(':')
                            .map(|part| part.to_string())
                            .collect_tuple::<(String, String)>()
                            .expect("Format must be valid")
                    })
                })
                .collect_vec()
        })
        .collect_vec();

    let mut valid = 0;

    for passport in &passports {
        if passport.len() == 8 {
            valid += 1;
        } else if passport.len() == 7 && !passport.iter().any(|(key, _value)| key.eq("cid")) {
            valid += 1;
        } else {
        }
    }

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
