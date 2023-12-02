use anyhow::bail;

use crate::solvers::AocDaySolver;

pub struct Day05Solver {}

impl Day05Solver {}

impl AocDaySolver for Day05Solver {
    fn part_a(&self, _input: &str) -> Result<String, anyhow::Error> {
        bail!("TODO")
    }

    fn part_b(&self, _input: &str) -> Result<String, anyhow::Error> {
        bail!("TODO")
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::init_test_log;
    use super::*;

    static _INPUT: &str = r#""#;

    #[test]
    fn part_a() {
        init_test_log();
        let _solver = Day05Solver {};
        assert!(false)
        // assert_eq!(solver.part_a(INPUT).unwrap(), 0.to_string());
    }

    #[test]
    fn part_b() {
        init_test_log();
        let _solver = Day05Solver {};
        assert!(false)
        // assert_eq!(solver.part_b(INPUT).unwrap(), 0.to_string());
    }
}