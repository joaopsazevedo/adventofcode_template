use crate::{solvers::AocDaySolver, AoCError};

pub mod day01_solver;
pub mod day02_solver;
pub mod day03_solver;
pub mod day04_solver;
pub mod day05_solver;
pub mod day06_solver;
pub mod day07_solver;
pub mod day08_solver;
pub mod day09_solver;
pub mod day10_solver;
pub mod day11_solver;
pub mod day12_solver;
pub mod day13_solver;
pub mod day14_solver;
pub mod day15_solver;
pub mod day16_solver;
pub mod day17_solver;
pub mod day18_solver;
pub mod day19_solver;
pub mod day20_solver;
pub mod day21_solver;
pub mod day22_solver;
pub mod day23_solver;
pub mod day24_solver;
pub mod day25_solver;

pub fn get_solver(day: usize) -> Result<Box<dyn AocDaySolver>, AoCError> {
    match day {
        01 => Ok(Box::new(day01_solver::Day01Solver {})),
        02 => Ok(Box::new(day02_solver::Day02Solver {})),
        03 => Ok(Box::new(day03_solver::Day03Solver {})),
        04 => Ok(Box::new(day04_solver::Day04Solver {})),
        05 => Ok(Box::new(day05_solver::Day05Solver {})),
        06 => Ok(Box::new(day06_solver::Day06Solver {})),
        07 => Ok(Box::new(day07_solver::Day07Solver {})),
        08 => Ok(Box::new(day08_solver::Day08Solver {})),
        09 => Ok(Box::new(day09_solver::Day09Solver {})),
        10 => Ok(Box::new(day10_solver::Day10Solver {})),
        11 => Ok(Box::new(day11_solver::Day11Solver {})),
        12 => Ok(Box::new(day12_solver::Day12Solver {})),
        13 => Ok(Box::new(day13_solver::Day13Solver {})),
        14 => Ok(Box::new(day14_solver::Day14Solver {})),
        15 => Ok(Box::new(day15_solver::Day15Solver {})),
        16 => Ok(Box::new(day16_solver::Day16Solver {})),
        17 => Ok(Box::new(day17_solver::Day17Solver {})),
        18 => Ok(Box::new(day18_solver::Day18Solver {})),
        19 => Ok(Box::new(day19_solver::Day19Solver {})),
        20 => Ok(Box::new(day20_solver::Day20Solver {})),
        21 => Ok(Box::new(day21_solver::Day21Solver {})),
        22 => Ok(Box::new(day22_solver::Day22Solver {})),
        23 => Ok(Box::new(day23_solver::Day23Solver {})),
        24 => Ok(Box::new(day24_solver::Day24Solver {})),
        25 => Ok(Box::new(day25_solver::Day25Solver {})),
        _ => Err(AoCError::InvalidDay(day)),
    }
}