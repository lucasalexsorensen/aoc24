// Directions
pub const UP: Point = Point { row: -1, col: 0 };
pub const DOWN: Point = Point { row: 1, col: 0 };
pub const LEFT: Point = Point { row: 0, col: -1 };
pub const RIGHT: Point = Point { row: 0, col: 1 };
pub const UP_LEFT: Point = Point { row: -1, col: -1 };
pub const UP_RIGHT: Point = Point { row: -1, col: 1 };
pub const DOWN_LEFT: Point = Point { row: 1, col: -1 };
pub const DOWN_RIGHT: Point = Point { row: 1, col: 1 };
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const DIAGONAL: [Point; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];
pub const ALL: [Point; 8] = [
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}
impl Point {
    pub const fn new(row: usize, col: usize) -> Self {
        Point {
            row: row as i32,
            col: col as i32,
        }
    }

    pub fn rotate_cw(&self) -> Self {
        Self {
            row: self.col,
            col: -self.row,
        }
    }

    pub fn rotate_ccw(&self) -> Self {
        Self {
            row: -self.col,
            col: self.row,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let row = self.row + other.row;
        let col = self.col + other.col;
        Point { row, col }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        let row = self.row - other.row;
        let col = self.col - other.col;
        Point { row, col }
    }
}
