use std::collections::HashMap;

pub fn run(puzzle: &str) -> Result<(), anyhow::Error> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> Result<i32, anyhow::Error> {
    let (mut left, mut right) = collect_cols(puzzle)?;

    left.sort();
    right.sort();

    Ok(left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum())
}

fn part2(puzzle: &str) -> Result<i32, anyhow::Error> {
    let (ids, tab) = collect_cols(puzzle)?;

    let mut freq: HashMap<i32, i32> = HashMap::new();
    for t in tab {
        *freq.entry(t).or_insert(0) += 1
    }

    Ok(ids
        .into_iter()
        .map(|id| id * freq.get(&id).unwrap_or(&0))
        .sum())
}

fn collect_cols(puzzle: &str) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    puzzle
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once("   ")
                .ok_or_else(|| anyhow::anyhow!("Invalid line"))?;
            Ok((left.parse::<i32>()?, right.parse::<i32>()?))
        })
        .collect::<Result<Vec<(i32, i32)>, _>>()
        .map(|pairs| pairs.into_iter().unzip())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 31);
    }
}
