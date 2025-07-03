//! Stupid brute force solution that will potentially check all 2^n combinations (3^n for part 2)
use itertools::Itertools;

fn main() {
    let input = include_str!("../../../data/d07.txt");
    let parsed_input = parse_input(input);
    println!("part one: {}", part_one(&parsed_input));
    println!("part two: {}", part_two(&parsed_input));
}

type ParsedInput = Vec<(usize, Vec<usize>)>;
fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let (goal, rest) = line.split_once(": ").expect("invalid line");
            let goal = goal.parse::<usize>().expect("goal is not a number");
            let terms = rest
                .split_whitespace()
                .map(|t| t.parse::<usize>().expect("term is not a number"))
                .collect::<Vec<_>>();
            (goal, terms)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn is_sequence_solvable(goal: &usize, terms: &[usize], operators: &[Operator]) -> bool {
    let op_combinations = std::iter::repeat_n(operators, terms.len() - 1).multi_cartesian_product();

    for combination in op_combinations {
        let mut r = terms[0];
        for (op, term) in combination.iter().zip(terms.iter().skip(1)) {
            r = match op {
                Operator::Add => r + term,
                Operator::Mul => r * term,
                Operator::Concat => {
                    let s = format!("{}{}", r, term);
                    s.parse::<usize>().expect("concatenation is not a number")
                }
            };
        }
        if r == *goal {
            return true;
        }
    }

    false
}

fn part_one(parsed_input: &ParsedInput) -> usize {
    let mut result = 0;
    let operators = [Operator::Add, Operator::Mul];
    for (goal, terms) in parsed_input {
        if is_sequence_solvable(goal, terms, &operators) {
            result += goal;
        }
    }

    result
}

fn part_two(parsed_input: &ParsedInput) -> usize {
    let mut result = 0;
    let operators = [Operator::Add, Operator::Mul, Operator::Concat];
    for (goal, terms) in parsed_input {
        if is_sequence_solvable(goal, terms, &operators) {
            result += goal;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d07.txt");

    #[test]
    fn test_part_one() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(part_one(&parsed_input), 3749);
    }

    #[test]
    fn test_part_two() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(part_two(&parsed_input), 11387);
    }
}
