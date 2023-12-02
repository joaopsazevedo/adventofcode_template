mod aoc_2015;
mod aoc_2016;
mod aoc_2017;
mod aoc_2018;
mod aoc_2019;
mod aoc_2020;
mod aoc_2021;
mod aoc_2022;
mod aoc_2023;
mod solvers;
mod utils;

use clap::{arg, command, Parser};
use env_logger::Env;
use log::{error, info};
use solvers::AocDaySolver;
use std::{fs::read_to_string, path::Path};
use thiserror::Error;

const VALID_DAYS: std::ops::RangeInclusive<usize> = 1..=25;
const VALID_YEARS: std::ops::RangeInclusive<usize> = 2015..=2023;

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("Year {0} is not in the valid range ({}-{})", VALID_YEARS.start(), VALID_YEARS.end())]
    InvalidYear(usize),
    #[error("Day {0} is not in the valid range ({}-{})", VALID_DAYS.start(), VALID_DAYS.end())]
    InvalidDay(usize),
    #[error("Invalid input. {0}")]
    InvalidInput(String),
    #[error("{0}")]
    NotImplementedYet(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Advent Of Code year between 2015 and 2023
    year: usize,

    /// Advent of Code day between 1 and 25
    #[arg(short, long)]
    day: Option<usize>,

    /// Path to an input file
    #[arg(short, long)]
    input: Option<String>,
}

impl Args {
    fn validate(&self) -> Result<(), anyhow::Error> {
        if !VALID_YEARS.contains(&self.year) {
            return Err(AoCError::InvalidYear(self.year).into());
        }
        if let Some(day) = self.day {
            if !VALID_DAYS.contains(&day) {
                return Err(AoCError::InvalidDay(day).into());
            }
        }
        if let Some(input) = &self.input {
            if !Path::new(input).exists() {
                return Err(
                    AoCError::InvalidInput(format!("File `{}` does not exist", input)).into(),
                );
            }
        }

        Ok(())
    }
}

fn default_input(year: usize, day: usize) -> Result<String, anyhow::Error> {
    read_to_string(format!("inputs/AoC{}/year{}day{:02}.txt", year, year, day))
        .map_err(|err| err.into())
}

fn get_solver(year: usize, day: usize) -> Result<Box<dyn AocDaySolver>, AoCError> {
    match year {
        2015 => aoc_2015::get_solver(day),
        2016 => aoc_2016::get_solver(day),
        2017 => aoc_2017::get_solver(day),
        2018 => aoc_2018::get_solver(day),
        2019 => aoc_2019::get_solver(day),
        2020 => aoc_2020::get_solver(day),
        2021 => aoc_2021::get_solver(day),
        2022 => aoc_2022::get_solver(day),
        2023 => aoc_2023::get_solver(day),
        _ => Err(AoCError::InvalidYear(year)),
    }
}

fn solve(year: usize, day: Option<usize>, input: Option<String>) -> Result<(), AoCError> {
    if let Some(day) = day {
        info!("Solving day {}", day);

        let solver = get_solver(year, day)?;

        let input = if let Some(input) = input {
            input
        } else {
            default_input(year, day)?
        };

        let (part_a_result, part_b_result) = (solver.part_a(&input), solver.part_b(&input));

        // TODO: Rewrite this if let as a match

        if let Ok(result) = part_a_result {
            info!("Part A Result: {result}");
        } else {
            error!("Part A Failed: {:?}", part_a_result.unwrap_err())
        }

        if let Ok(result) = part_b_result {
            info!("Part B Result: {result}");
        } else {
            error!("Part B Failed: {:?}", part_b_result.unwrap_err())
        }
    } else {
        return Err(AoCError::NotImplementedYet(String::from(
            "All days solver not implemented yet",
        )));
    }

    Result::<(), AoCError>::Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .parse_env(Env::default().default_filter_or(log::Level::Debug.as_str()))
        .format_timestamp(None)
        .init();

    let args = Args::parse();

    info!("Advent of Code {}", args.year);

    args.validate()?;

    let result = solve(args.year, args.day, args.input);

    if let Err(err) = result {
        error!("{:?}", err)
    }

    Ok(())
}
