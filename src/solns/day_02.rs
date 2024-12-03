pub fn run(puzzle: &str) -> anyhow::Result<()> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    Ok(parse(puzzle)?
        .iter()
        .map(|report| is_report_safe(report) as i32)
        .sum())
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    Ok(parse(puzzle)?
        .iter()
        .map(|report| {
            if is_report_safe(report) {
                return 1;
            }

            for idx in 0..report.len() {
                if is_report_safe(&report_with_level_removed(report, idx)) {
                    return 1;
                }
            }

            return 0;
        })
        .sum())
}

fn parse(puzzle: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    puzzle
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()
        .map_err(|e| anyhow::anyhow!(e))
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let (all_positive, all_negative, all_in_range) = report
        // Find the difference between each pair.
        .windows(2)
        .map(|w| w[0] - w[1])
        // Map every difference to a tuple of booleans (is_pos, is_neg, is_in_range).
        .map(|diff| {
            (
                diff > 0,                           // Is positive
                diff < 0,                           // Is negative
                diff.abs() >= 1 && diff.abs() <= 3, // Is in range
            )
        })
        // Fold map to check (are_all_positive, are_all_negative, are_all_in_range).
        .fold((true, true, true), |acc, (is_pos, is_neg, is_in_range)| {
            (
                acc.0 && is_pos,      // All are positive
                acc.1 && is_neg,      // All are negative
                acc.2 && is_in_range, // All are in range
            )
        });

    (all_positive || all_negative) && all_in_range
}

fn report_with_level_removed(report: &[i32], idx: usize) -> Vec<i32> {
    report
        .iter()
        .enumerate()
        .filter_map(|(i, &val)| if i == idx { None } else { Some(val) })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 4);
    }
}
