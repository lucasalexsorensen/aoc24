use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../../data/d02.txt");

    println!("part one: {}", part_one(input)?);
    println!("part two: {}", part_two(input)?);

    Ok(())
}

type ParsedInput = Vec<Vec<usize>>;

fn parse_input(input: &str) -> Result<ParsedInput> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split_whitespace()
                .map(|part| part.parse::<usize>().unwrap())
                .collect();
            Ok(parts)
        })
        .collect()
}

fn part_one(input: &str) -> Result<usize> {
    let parsed_input = parse_input(input)?;

    let result = parsed_input.iter().filter(|x| is_valid(x)).count();
    Ok(result)
}

fn part_two(input: &str) -> Result<usize> {
    let parsed_input = parse_input(input)?;

    let result = parsed_input
        .iter()
        .filter(|x| {
            is_valid(*x)
                || (0..x.len()).any(|i| {
                    let slice1 = &x[..i];
                    let slice2 = &x[(i + 1)..];
                    let combined = [slice1, slice2].concat();
                    is_valid(&combined)
                })
        })
        .count();
    Ok(result)
}

fn is_valid(x: &[usize]) -> bool {
    let diff: Vec<_> = x
        .windows(2)
        .map(|w| w[0] as isize - w[1] as isize)
        .collect::<Vec<_>>();
    // check monotonicity
    let n_pos = diff.iter().filter(|d| **d > 0).count();
    let n_neg = diff.iter().filter(|d| **d < 0).count();
    let is_monotone = n_pos == diff.len() || n_neg == diff.len();
    // check max diffs
    let is_max_diff_valid = diff.iter().all(|d| 1 <= d.abs() && d.abs() <= 3);
    is_monotone && is_max_diff_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d02.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT).unwrap(), 4);
    }
}
