//! Smart solution that works backwards from the goal, and continues only with the options that are possible

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

fn valid(v: usize, terms: &[usize], term_idx: usize, operators: &[Operator]) -> bool {
    if term_idx == 0 {
        return terms[term_idx] == v;
    }

    let t = terms[term_idx];
    let next_idx = term_idx - 1;

    operators.iter().any(|op| match op {
        Operator::Add => {
            if v < t {
                return false;
            }
            valid(v - t, terms, next_idx, operators)
        }
        Operator::Mul => {
            if v % t != 0 {
                return false;
            }
            valid(v / t, terms, next_idx, operators)
        }
        Operator::Concat => {
            let next_power = get_next_power(t);
            let q = v / next_power;
            let r = v % next_power;
            if r != t {
                return false;
            }
            valid(q, terms, next_idx, operators)
        }
    })
}

const fn get_next_power(x: usize) -> usize {
    if x < 10 {
        10
    } else if x < 100 {
        100
    } else if x < 1000 {
        1000
    } else if x < 10000 {
        10000
    } else {
        panic!("x is too large");
    }
}

fn part_one(parsed_input: &ParsedInput) -> usize {
    let mut result = 0;
    let operators = [Operator::Add, Operator::Mul];
    for (goal, terms) in parsed_input {
        if valid(*goal, terms, terms.len() - 1, &operators) {
            result += goal;
        }
    }

    result
}

fn part_two(parsed_input: &ParsedInput) -> usize {
    let mut result = 0;
    let operators = [Operator::Add, Operator::Mul, Operator::Concat];
    for (goal, terms) in parsed_input {
        if valid(*goal, terms, terms.len() - 1, &operators) {
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
