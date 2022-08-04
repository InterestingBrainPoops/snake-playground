use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs = match rhs {
            Direction::Up => Coordinate::new(0, 1),
            Direction::Down => Coordinate::new(0, -1),
            Direction::Left => Coordinate::new(-1, 0),
            Direction::Right => Coordinate::new(1, 0),
        };
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}
