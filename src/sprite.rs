use std::{collections::HashSet, cmp::max};
use crate::{position::{Point, IndexType, ToPoint}, canvas::Canvas, drawable::Drawable};

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
