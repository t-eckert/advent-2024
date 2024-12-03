use crate::solns::*;

pub fn run(day: u8) -> Result<(), anyhow::Error> {
    match day {
        1 => day_01::run(include_str!("../puzzles/day_01.txt")),
        2 => day_02::run(include_str!("../puzzles/day_02.txt")),
        3 => day_03::run(include_str!("../puzzles/day_03.txt")),
        _ => Err(anyhow::anyhow!("Day {} not implemented", day)),
    }
}
