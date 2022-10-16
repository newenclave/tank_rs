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
    pub fn distance_as_f32(&self, other: &Point) -> f32 {
        let delta_x = self.x as f32 - other.x as f32;
        let delta_y = self.y as f32 - other.y as f32;
        (delta_x * delta_x + delta_y * delta_y).sqrt()
    }

    pub fn move_to(&mut self, x: IndexType, y: IndexType) {
        self.x = x;
        self.y = y;
    }
}

pub trait AsPoint {
    fn as_point(&self) -> Point;
}

impl AsPoint for (IndexType, IndexType) {
    fn as_point(&self) -> Point {
        Point::new(self.0, self.1)
    }
}
