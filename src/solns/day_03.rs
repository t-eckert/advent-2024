use regex::Regex;

pub fn run(puzzle: &str) -> anyhow::Result<()> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)")?;

    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(puzzle).map(|c| c.extract()) {
        total = total + a.parse::<i32>()? * b.parse::<i32>()?;
    }

    Ok(total)
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|do\(\)|don't\(\)")?;

    let mut total = 0;
    let mut is_active = true;
    for cap in re.captures_iter(puzzle) {
        match parse_op(&cap.get(0).ok_or(anyhow::anyhow!("No match found."))?)? {
            Operation::Do => is_active = true,
            Operation::Dont => is_active = false,
            Operation::Mul => {
                let a = cap
                    .get(1)
                    .ok_or(anyhow::anyhow!("Multiply op does not have first value."))?
                    .as_str();
                let b = cap
                    .get(2)
                    .ok_or(anyhow::anyhow!("Multiple op does not have second value."))?
                    .as_str();
                if is_active {
                    total = total + a.parse::<i32>()? * b.parse::<i32>()?;
                }
            }
        }
    }

    Ok(total)
}

enum Operation {
    Do,
    Dont,
    Mul,
}

fn parse_op(raw_op: &regex::Match) -> anyhow::Result<Operation> {
    if raw_op.as_str() == "do()" {
        return Ok(Operation::Do);
    } else if raw_op.as_str() == "don't()" {
        return Ok(Operation::Dont);
    } else if raw_op.as_str().contains("mul") {
        return Ok(Operation::Mul);
    }

    return Err(anyhow::anyhow!(
        "Operation cannot be categorized {}",
        raw_op.as_str()
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    pub const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_1).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_2).unwrap(), 48);
    }
}
