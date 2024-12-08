use std::{cmp::Ordering, collections::HashMap};

pub fn run(puzzle: &str) -> anyhow::Result<()> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    let (rules, updates) = parse(puzzle)?;

    Ok(updates
        .into_iter()
        .filter_map(|update| {
            if is_update_valid(&rules, &update) {
                Some(middle(update))
            } else {
                None
            }
        })
        .sum())
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    let (rules, updates) = parse(puzzle)?;

    Ok(updates
        .into_iter()
        .filter_map(|update| {
            if !is_update_valid(&rules, &update) {
                Some(middle(fix_page(&rules, &update)))
            } else {
                None
            }
        })
        .sum())
}

fn parse(puzzle: &str) -> anyhow::Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];

    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        } else if let Some((first, second)) = parse_rule(line) {
            // Handle the rule lines.
            rules
                .entry(first)
                .and_modify(|following| following.push(second))
                .or_insert(vec![second]);
        } else {
            // Handle the update lines.
            updates.push(
                line.split(",")
                    .map(|x| Ok(x.parse::<i32>()?))
                    .collect::<anyhow::Result<Vec<i32>>>()?,
            );
        };
    }

    Ok((rules, updates))
}

fn parse_rule(rule: &str) -> Option<(i32, i32)> {
    rule.split_once("|")
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
}

fn is_update_valid(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut current = update[0];
    let mut preceeding = &update[0..0];
    let mut following = &update[1..];

    while preceeding.len() < update.len() {
        if let Some(following_rules) = rules.get(&current) {
            // The update is not valid if the values preceeding the current one
            // include a value that the rule deems must follow the value.
            if following_rules.iter().any(|rule| preceeding.contains(rule)) {
                return false;
            }
        }

        // Shift forward to check the next value.
        preceeding = &update[0..(preceeding.len() + 1)];
        if let Some(page) = following.get(0) {
            current = *page;
            following = &following[1..];
        }
    }

    true
}

fn middle(page: Vec<i32>) -> i32 {
    page[page.len() / 2]
}

fn fix_page(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut fixed_update = update.clone();
    fixed_update.sort_by(|a, b| {
        if rules.get(a).is_some_and(|pages| pages.contains(b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    fixed_update
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 123);
    }
}
