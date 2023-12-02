pub trait AocDaySolver {
    fn part_a(&self, input: &str) -> Result<String, anyhow::Error>;
    fn part_b(&self, input: &str) -> Result<String, anyhow::Error>;
}
