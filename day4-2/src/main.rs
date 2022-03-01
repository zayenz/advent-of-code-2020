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
use std::fs::read_to_string;
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
        let is_valid = validate_passport(passport);
        if is_valid {
            valid += 1;
        }
    }

    Ok(valid)
}

fn validate_passport(passport: &Vec<(String, String)>) -> bool {
    if passport.len() < 7 {
        return false;
    }

    let data = passport
        .iter()
        .cloned()
        .collect::<HashMap<String, String>>();

    if let Some(val) = data.get(&"byr".to_string()) {
        if let Ok(year) = val.parse::<usize>() {
            if year < 1920 || 2002 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"iyr".to_string()) {
        if let Ok(year) = val.parse::<usize>() {
            if year < 2010 || 2020 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"eyr".to_string()) {
        if let Ok(year) = val.parse::<usize>() {
            if year < 2020 || 2030 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"hgt".to_string()) {
        if !(val.ends_with("cm") || val.ends_with("in")) {
            return false;
        }
        //errer
        if let Ok(year) = val
            .trim_end_matches(char::is_ascii_alphabetic())
            .parse::<usize>()
        {
            if year < 1920 || 2002 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"hcl".to_string()) {
        if val.starts_with('#') {
            if let Ok(year) = val.parse::<usize>() {
                if year < 1920 || 2002 < year {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"byr".to_string()) {
        if let Ok(year) = val.parse::<usize>() {
            if year < 1920 || 2002 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    if let Some(val) = data.get(&"byr".to_string()) {
        if let Ok(year) = val.parse::<usize>() {
            if year < 1920 || 2002 < year {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    true
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
