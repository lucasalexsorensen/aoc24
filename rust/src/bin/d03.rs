use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("../../../data/d03.txt");

    println!("part one: {}", part_one(input)?);
    println!("part two: {}", part_two(input)?);

    Ok(())
}

fn part_one(input: &str) -> Result<usize> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let result = pattern
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum::<usize>();

    Ok(result)
}

fn part_two(input: &str) -> Result<usize> {
    let pattern =
        Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)|(?<enable>do)\(\)|(?<disable>don't)\(\)")?;

    let mut result = 0;
    let mut enabled = true;
    for caps in pattern.captures_iter(input) {
        let a = caps.name("a");
        let b = caps.name("b");
        let enable = caps.name("enable");
        let disable = caps.name("disable");

        if enable.is_some() {
            enabled = true;
            continue;
        }
        if disable.is_some() {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }

        result += a.unwrap().as_str().parse::<usize>()? * b.unwrap().as_str().parse::<usize>()?
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(include_str!("../../../test-data/d03a.txt")).unwrap(),
            161
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(include_str!("../../../test-data/d03b.txt")).unwrap(),
            48
        );
    }
}
