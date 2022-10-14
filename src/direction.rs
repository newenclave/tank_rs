use crate::position::{Point, AsPoint};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up, Right, Down, Left,
}

impl Direction {
    pub fn go_forward(&self, p: Point) -> Point {
        match *self {
            Direction::Up => (p.x, p.y - 1).as_point(),
            Direction::Right => (p.x + 1, p.y).as_point(),
            Direction::Down => (p.x, p.y + 1).as_point(),
            Direction::Left => (p.x - 1, p.y).as_point(),
        }
    }
    pub fn go_back(&self, p: Point) -> Point {
        match *self {
            Direction::Up => (p.x, p.y + 1).as_point(),
            Direction::Right => (p.x - 1, p.y).as_point(),
            Direction::Down => (p.x, p.y - 1).as_point(),
            Direction::Left => (p.x + 1, p.y).as_point(),
        }
    }
    pub fn next_clockwise(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

