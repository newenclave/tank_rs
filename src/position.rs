
pub type IndexType = i16;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: IndexType,
    pub y: IndexType,
}

impl Point {
    pub fn new(x: IndexType, y: IndexType) -> Self {
        Self { x, y }
    }
}

pub trait ToPoint {
    fn as_point(&self) -> Point;
}

impl ToPoint for (IndexType, IndexType) {
    fn as_point(&self) -> Point {
        Point::new(self.0, self.1)
    }
}

