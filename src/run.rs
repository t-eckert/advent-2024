pub fn run(day: u8) -> Result<(), anyhow::Error> {
    match day {
        1 => crate::solns::day_01::run(include_str!("../puzzles/day_01.txt")),
        2 => crate::solns::day_02::run(include_str!("../puzzles/day_02.txt")),
        _ => Err(anyhow::anyhow!("Day {} not implemented", day)),
    }
}
