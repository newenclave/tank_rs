use std::{collections::HashSet, cmp::max};
use crate::{position::{Point, IndexType, AsPoint}, canvas::Canvas, drawable::Drawable};

pub struct Sprite {
    values: HashSet<Point>,
    max: Point,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
            max: Point::new(0, 0),
        }
    }
    pub fn get_width(&self) -> IndexType {
        self.max.x + 1
    }
    pub fn get_height(&self) -> IndexType {
        self.max.y + 1
    }
}

impl Canvas for Sprite {
    fn draw_dot(&mut self, x: IndexType, y: IndexType) -> bool {
        self.values.insert((x, y).as_point());
        self.max = (max(x, self.max.x), max(y, self.max.y)).as_point();
        true
    }
}

impl Drawable for Sprite {
    fn draw(&self, canvas: &mut dyn Canvas) {
        for p in self.values.iter() {
            canvas.draw_dot(p.x, p.y);
        }
    }
}
