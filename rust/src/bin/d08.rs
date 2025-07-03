use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc2024::utils::{grid::Grid, point::Point};

fn main() {
    let input = include_str!("../../../data/d08.txt");
    let grid = Grid::parse(input);
    let grouped_antennas = group_antennas(&grid);

    println!("part one: {}", part_one(&grid, &grouped_antennas));
    println!("part two: {}", part_two(&grid, &grouped_antennas));
}

fn group_antennas(grid: &Grid<u8>) -> HashMap<u8, Vec<Point>> {
    grid.iter()
        .filter(|(_, c)| *c != b'.')
        .fold(HashMap::new(), |mut acc, (pos, c)| {
            acc.entry(c).or_default().push(pos);
            acc
        })
}

fn part_one(grid: &Grid<u8>, grouped_antennas: &HashMap<u8, Vec<Point>>) -> usize {
    let mut antinodes = HashSet::new();
    for (_, positions) in grouped_antennas.iter() {
        positions.iter().combinations(2).for_each(|comb| {
            let (a, b) = (comb[0], comb[1]);
            let diff = (*b) - (*a);
            let first_node = *a + (diff * 2);

            if grid.get(first_node).is_some() {
                antinodes.insert(first_node);
            }

            let second_node = *a - diff;
            if grid.get(second_node).is_some() {
                antinodes.insert(second_node);
            }
        })
    }

    antinodes.len()
}

fn part_two(grid: &Grid<u8>, grouped_antennas: &HashMap<u8, Vec<Point>>) -> usize {
    let mut antinodes = HashSet::new();
    for (_, positions) in grouped_antennas.iter() {
        positions.iter().combinations(2).for_each(|comb| {
            let (a, b) = (comb[0], comb[1]);
            let diff = (*b) - (*a);

            // check the first direction
            let mut antinode = *a + diff;
            while grid.get(antinode).is_some() {
                antinodes.insert(antinode);
                antinode = antinode + diff;
            }

            // check the other direction
            let mut antinode = *b - diff;
            while grid.get(antinode).is_some() {
                antinodes.insert(antinode);
                antinode = antinode - diff;
            }
        })
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d08.txt");

    #[test]
    fn test_part_one() {
        let grid = Grid::parse(TEST_INPUT);
        let grouped_antennas = group_antennas(&grid);
        assert_eq!(part_one(&grid, &grouped_antennas), 14);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::parse(TEST_INPUT);
        let grouped_antennas = group_antennas(&grid);
        assert_eq!(part_two(&grid, &grouped_antennas), 34);
    }
}
