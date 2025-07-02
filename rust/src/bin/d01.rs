use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../../data/d01.txt");

    println!("part one: {}", part_one(input)?);
    println!("part two: {}", part_two(input)?);

    Ok(())
}

type ParsedInput = (Vec<usize>, Vec<usize>);

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut iterator = line.split_whitespace();
            let a: usize = iterator.next().unwrap().parse().unwrap();
            let b: usize = iterator.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip()
}

fn part_one(input: &str) -> Result<usize> {
    let (mut x1, mut x2) = parse_input(input);

    x1.sort();
    x2.sort();

    let result = x1
        .iter()
        .zip(x2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>();

    Ok(result)
}

fn part_two(input: &str) -> Result<usize> {
    let (x1, x2) = parse_input(input);
    let x2_counter = count_items(&x2);

    let result = x1
        .iter()
        .zip(x2.iter())
        .map(|(a, _)| a * x2_counter.get(a).unwrap_or(&0))
        .sum::<usize>();

    Ok(result)
}

fn count_items<T: Eq + std::hash::Hash>(items: &[T]) -> HashMap<&T, usize> {
    let mut counter = HashMap::new();
    for item in items {
        *counter.entry(item).or_insert(0) += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d01.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT).unwrap(), 31);
    }
}
