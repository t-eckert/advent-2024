pub const TEMPLATE: &str = r#"pub fn run(puzzle: &str) -> anyhow::Result<()> {
    part1(puzzle)?;
    part2(puzzle)?;

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    Ok(0)
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), -1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), -1);
    }
}"#;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
