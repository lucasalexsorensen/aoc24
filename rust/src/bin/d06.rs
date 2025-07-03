use itertools::iproduct;
use std::collections::HashSet;

use anyhow::Result;
use aoc2024::utils::{
    grid::Grid,
    point::{Point, UP},
};

fn main() -> Result<()> {
    let input = include_str!("../../../data/d06.txt");

    let grid = Grid::parse(input);

    println!("part one: {}", part_one(&grid)?);
    println!("part two: {}", part_two(&grid)?);

    Ok(())
}

fn part_one(grid: &Grid<u8>) -> Result<usize> {
    let mut pos = grid.find_first(|&c| c == b'^').unwrap();
    let mut dir = UP;

    let mut visited: HashSet<Point> = HashSet::new();
    loop {
        visited.insert(pos);
        match grid.get(pos + dir) {
            Some(b'#') => {
                dir = dir.rotate_cw();
                pos = pos + dir;
            }
            Some(_) => {
                pos = pos + dir;
            }
            None => break,
        }
    }

    Ok(visited.len())
}

fn generate_possible_grids(grid: &Grid<u8>) -> impl Iterator<Item = Grid<u8>> {
    let coord_iterator = iproduct!(0..grid.height, 0..grid.width);
    coord_iterator.filter_map(|(row, col)| {
        let pos = Point::new(row, col);
        let tile = grid.get(pos);
        match tile {
            Some(b'#') | Some(b'^') | None => None,
            Some(_) => {
                let mut new_grid = grid.clone();
                new_grid[pos] = b'#';
                Some(new_grid)
            }
        }
    })
}

fn does_grid_end_in_loop(grid: &Grid<u8>, start_pos: Point) -> bool {
    let mut pos = start_pos;
    let mut dir = UP;
    let mut seen: HashSet<(Point, Point)> = HashSet::new();

    loop {
        if seen.contains(&(pos, dir)) {
            return true;
        }
        seen.insert((pos, dir));

        match grid.get(pos + dir) {
            Some(b'#') => {
                dir = dir.rotate_cw();
                pos = pos + dir;
            }
            Some(_) => {
                pos = pos + dir;
            }
            None => break,
        }
    }

    false
}

fn part_two(grid: &Grid<u8>) -> Result<usize> {
    let start_pos = grid.find_first(|&c| c == b'^').unwrap();
    let result = generate_possible_grids(grid)
        .filter(|g| does_grid_end_in_loop(g, start_pos))
        .count();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d06.txt");

    #[test]
    fn test_part_one() {
        let grid = Grid::parse(TEST_INPUT);
        assert_eq!(part_one(&grid).unwrap(), 41);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::parse(TEST_INPUT);
        assert_eq!(part_two(&grid).unwrap(), 6);
    }
}
