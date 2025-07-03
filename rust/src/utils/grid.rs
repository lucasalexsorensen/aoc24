use super::point::Point;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagonalDirection {
    TopLeftToBottomRight,
    TopRightToBottomLeft,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub height: usize,
    pub width: usize,
    pub data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Point, T)> {
        (0..self.height).flat_map(move |row| {
            (0..self.width).map(move |col| (Point::new(row, col), self[Point::new(row, col)]))
        })
    }

    pub fn find_first(&self, predicate: impl Fn(&T) -> bool) -> Option<Point> {
        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point::new(row, col);
                if predicate(&self[point]) {
                    return Some(point);
                }
            }
        }
        None
    }

    pub fn get(&self, point: Point) -> Option<T> {
        if !(0..self.height).contains(&(point.row as usize))
            || !(0..self.width).contains(&(point.col as usize))
        {
            return None;
        }
        Some(self.data[self.width * point.row as usize + point.col as usize])
    }

    pub fn get_column(&self, col: usize) -> Vec<T> {
        (0..self.height)
            .map(|row| self[Point::new(row, col)])
            .collect()
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        let start = row * self.width;
        let stop = (row + 1) * self.width;
        &self.data[start..stop]
    }

    /// Get all diagonals of the grid in the specified direction.
    ///
    /// For TopRightToBottomLeft direction:
    /// The diagonals are returned from the top-right corner to the bottom-left corner
    /// Example:
    /// [a, b, c,
    ///  d, e, f,
    ///  g, h, i]
    /// => [[c], [b, f], [a, e, i], [d, h], [g]]
    ///
    /// For TopLeftToBottomRight direction:
    /// The diagonals are returned from the top-left corner to the bottom-right corner
    /// Example:
    /// [a, b, c,
    ///  d, e, f,
    ///  g, h, i]
    /// => [[a], [d, b], [g, e, c], [h, f], [i]]
    pub fn get_diagonals(&self, direction: DiagonalDirection) -> Vec<Vec<T>> {
        match direction {
            DiagonalDirection::TopRightToBottomLeft => self.get_diagonals_ne_sw(),
            DiagonalDirection::TopLeftToBottomRight => self.get_diagonals_nw_se(),
        }
    }

    /// Get diagonals from top-right to bottom-left (northeast to southwest)
    fn get_diagonals_ne_sw(&self) -> Vec<Vec<T>> {
        let mut diagonals = Vec::new();

        // Start from top row, right to left
        for start_col in (0..self.width).rev() {
            let mut diagonal = Vec::new();
            let mut row = 0;
            let mut col = start_col;

            while row < self.height && col < self.width {
                diagonal.push(self[Point::new(row, col)]);
                row += 1;
                col += 1;
            }
            diagonals.push(diagonal);
        }

        // Start from left column, top to bottom (excluding top-left corner which was already processed)
        for start_row in 1..self.height {
            let mut diagonal = Vec::new();
            let mut row = start_row;
            let mut col = 0;

            while row < self.height && col < self.width {
                diagonal.push(self[Point::new(row, col)]);
                row += 1;
                col += 1;
            }
            diagonals.push(diagonal);
        }

        diagonals
    }

    /// Get diagonals from top-left to bottom-right (northwest to southeast)
    fn get_diagonals_nw_se(&self) -> Vec<Vec<T>> {
        let mut diagonals = Vec::new();

        // Start from top row, left to right
        for start_col in 0..self.width {
            let mut diagonal = Vec::new();
            let mut row = 0;
            let mut col = start_col;

            while row < self.height && col < self.width {
                diagonal.push(self[Point::new(row, col)]);
                row += 1;
                col += 1;
            }
            diagonals.push(diagonal);
        }

        // Start from left column, top to bottom (excluding top-left corner which was already processed)
        for start_row in 1..self.height {
            let mut diagonal = Vec::new();
            let mut row = start_row;
            let mut col = 0;

            while row < self.height && col < self.width {
                diagonal.push(self[Point::new(row, col)]);
                row += 1;
                col += 1;
            }
            diagonals.push(diagonal);
        }

        diagonals
    }
}

impl Grid<u8> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut data = Vec::with_capacity(width * height);
        raw.iter().for_each(|slice| data.extend_from_slice(slice));
        Grid {
            height,
            width,
            data,
        }
    }

    pub fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point::new(row, col);
                print!("{}", self[point] as char);
            }
            println!();
        }
        println!();
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[self.width * index.row as usize + index.col as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[self.width * index.row as usize + index.col as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut grid = Grid::parse("123\n456\n789");
        assert_eq!(grid.height, 3);
        assert_eq!(grid.width, 3);
        assert_eq!(grid[Point::new(0, 0)], b'1');
        assert_eq!(grid[Point::new(0, 2)], b'3');
        assert_eq!(grid[Point::new(1, 1)], b'5');
        assert_eq!(grid[Point::new(2, 0)], b'7');
        assert_eq!(grid[Point::new(2, 2)], b'9');

        grid[Point::new(0, 0)] = b'x';
        assert_eq!(grid[Point::new(0, 0)], b'x');
    }

    // :)

    #[test]
    fn test_get_column() {
        let grid = Grid::parse("123\n456\n789");
        assert_eq!(grid.get_column(0), b"147");
        assert_eq!(grid.get_column(1), b"258");
        assert_eq!(grid.get_column(2), b"369");
    }

    #[test]
    fn test_get_row() {
        let grid = Grid::parse("123\n456\n789");
        assert_eq!(grid.get_row(0), b"123");
        assert_eq!(grid.get_row(1), b"456");
        assert_eq!(grid.get_row(2), b"789");
    }

    #[test]
    fn test_get_diagonals() {
        let grid = Grid::parse("abc\ndef\nghi");
        assert_eq!(
            grid.get_diagonals(DiagonalDirection::TopRightToBottomLeft),
            vec![
                "c".as_bytes().to_vec(),
                "bf".as_bytes().to_vec(),
                "aei".as_bytes().to_vec(),
                "dh".as_bytes().to_vec(),
                "g".as_bytes().to_vec(),
            ]
        );

        assert_eq!(
            grid.get_diagonals(DiagonalDirection::TopLeftToBottomRight),
            vec![
                "aei".as_bytes().to_vec(),
                "bf".as_bytes().to_vec(),
                "c".as_bytes().to_vec(),
                "dh".as_bytes().to_vec(),
                "g".as_bytes().to_vec(),
            ]
        );
    }
}
