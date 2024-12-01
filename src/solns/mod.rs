pub const TEMPLATE: &str = r#"
use bytes::Bytes;

pub fn run(puzzle: &Bytes) -> Result<(), anyhow::Error> {
    part1(puzzle)?;
    part2(puzzle)?;
    Ok(())
}

fn part1(puzzle: &Bytes) -> Result<(), anyhow::Error> {
    Ok(())
}

fn part2(puzzle: &Bytes) -> Result<(), anyhow::Error> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &[u8] = b"";
}"#;

pub mod day_01;
