use std::time::Instant;

use crate::solns::*;

pub fn run(day: u8) -> Result<(), anyhow::Error> {
    let now = Instant::now();

    match day {
        1 => day_01::run(include_str!("../puzzles/day_01.txt")),
        2 => day_02::run(include_str!("../puzzles/day_02.txt")),
        3 => day_03::run(include_str!("../puzzles/day_03.txt")),
        4 => day_04::run(include_str!("../puzzles/day_04.txt")),
        5 => day_05::run(include_str!("../puzzles/day_05.txt")),
        6 => day_06::run(include_str!("../puzzles/day_06.txt")),
        _ => Err(anyhow::anyhow!("Day {} not implemented", day)),
    }?;

    println!("Ran in {:?}", now.elapsed());

    Ok(())
}
