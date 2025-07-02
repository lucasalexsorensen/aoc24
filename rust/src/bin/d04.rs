use anyhow::Result;
use aoc2024::utils::{
    grid::{DiagonalDirection, Grid},
    point::{DOWN, LEFT, Point, RIGHT, UP},
};

fn main() -> Result<()> {
    let input = include_str!("../../../data/d04.txt");

    println!("part one: {}", part_one(input)?);
    println!("part two: {}", part_two(input)?);

    Ok(())
}

fn slice_match(v: &[u8]) -> bool {
    v == b"XMAS" || v == b"SAMX"
}

fn part_one(input: &str) -> Result<usize> {
    let grid = Grid::parse(input);

    let mut result = 0;

    let columns = (0..grid.width).map(|c| grid.get_column(c));
    let rows = (0..grid.height).map(|r| grid.get_row(r).to_vec());
    let diagonals_nw_se = grid.get_diagonals(DiagonalDirection::TopLeftToBottomRight);
    let diagonals_ne_sw = grid.get_diagonals(DiagonalDirection::TopRightToBottomLeft);

    columns
        .chain(rows)
        .chain(diagonals_ne_sw)
        .chain(diagonals_nw_se)
        .for_each(|slice| {
            slice.windows(4).for_each(|window| {
                if slice_match(window) {
                    result += 1;
                }
            });
        });

    Ok(result)
}

fn part_two(input: &str) -> Result<usize> {
    let grid = Grid::parse(input);

    let mut result = 0;

    // iterate all of the "inner" points of the grid
    for r in 1..grid.height - 1 {
        for c in 1..grid.width - 1 {
            let p = Point::new(r, c);
            if grid[p] != b'A' {
                continue;
            }

            let ul = grid[p + UP + LEFT];
            let ur = grid[p + UP + RIGHT];
            let dl = grid[p + DOWN + LEFT];
            let dr = grid[p + DOWN + RIGHT];

            // By checking to see if the difference is equal to the difference between S and M,
            // we can determine if the pattern is valid
            // Absolute difference => both directions are valid
            const VALID_DIFF: u8 = b'S' - b'M';

            if ul.abs_diff(dr) == VALID_DIFF && ur.abs_diff(dl) == VALID_DIFF {
                result += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../test-data/d04.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT).unwrap(), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT).unwrap(), 9);
    }
}
