use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = include_str!("../../../data/d05.txt");

    let parsed_input = parse_input(input)?;

    println!("part one: {}", part_one(&parsed_input)?);
    println!("part two: {}", part_two(&parsed_input)?);

    Ok(())
}

struct ParsedInput {
    pub rules: HashMap<usize, Vec<usize>>,
    pub sequences: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Result<ParsedInput> {
    match input.split_once("\n\n") {
        Some((rules_chunk, sequences_chunk)) => {
            let rules = rules_chunk.lines().fold(
                HashMap::new(),
                |mut acc: HashMap<usize, Vec<usize>>, line| {
                    let (k, v) = line.split_once("|").unwrap();
                    acc.entry(k.parse().unwrap())
                        .or_default()
                        .push(v.parse().unwrap());
                    acc
                },
            );

            let sequences = sequences_chunk
                .lines()
                .map(|line| {
                    line.split(",")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            Ok(ParsedInput { rules, sequences })
        }
        None => anyhow::bail!("Invalid input"),
    }
}

fn is_sequence_valid(sequence: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    let page_to_index_lookup: HashMap<usize, usize> = sequence
        .iter()
        .enumerate()
        .map(|(i, page)| (*page, i))
        .collect();

    for (i, page) in sequence.iter().enumerate() {
        let Some(rules_for_page) = rules.get(page) else {
            continue;
        };

        let has_violation = rules_for_page.iter().any(|should_preceed_page| {
            page_to_index_lookup
                .get(should_preceed_page)
                .is_some_and(|should_preceed_idx| i > *should_preceed_idx)
        });

        if has_violation {
            return false;
        }
    }

    true
}

fn part_one(parsed_input: &ParsedInput) -> Result<usize> {
    let result = parsed_input
        .sequences
        .iter()
        .filter(|seq| is_sequence_valid(seq, &parsed_input.rules))
        .map(|seq| {
            let mid = seq.len() / 2;
            seq[mid]
        })
        .sum();

    Ok(result)
}

fn part_two(parsed_input: &ParsedInput) -> Result<usize> {
    let mut invalid_sequences = parsed_input
        .sequences
        .iter()
        .filter(|seq| !is_sequence_valid(seq, &parsed_input.rules))
        .collect::<Vec<_>>();

    let result = invalid_sequences
        .iter()
        .map(|seq| {
            let mut seq_sorted = (*seq).clone();
            seq_sorted.sort_by(|a, b| {
                if let Some(rules_for_a) = parsed_input.rules.get(a) {
                    if rules_for_a.contains(b) {
                        return Ordering::Less;
                    }
                }
                Ordering::Greater
            });
            let mid = seq_sorted.len() / 2;
            seq_sorted[mid]
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d05.txt");

    #[test]
    fn test_part_one() {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part_one(&parsed_input).unwrap(), 143);
    }

    #[test]
    fn test_part_two() {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part_two(&parsed_input).unwrap(), 123);
    }
}
