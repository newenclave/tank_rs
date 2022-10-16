use crate::{
    canvas::Canvas,
    position::{AsPoint, IndexType, Point},
    point_set::PointSet
};
use std::{cmp::max, collections::HashSet};

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

    pub fn new_from_string(value: &str) -> Self {
        let mut instance = Self {
            values: HashSet::new(),
            max: Point::new(0, 0),
        };
        instance.draw_from_string(value);
        instance
    }

    pub fn rotate_90(&mut self) {
        let mut tmp = Self::new();
        for p in self.values.iter() {
            tmp.draw_dot(self.max.y - p.y, p.x);
        }
        self.values = tmp.values;
        self.max = tmp.max;
    }

    pub fn draw_to_canvas(&self, canvas: &mut dyn Canvas, x: IndexType, y: IndexType) {
        for p in self.values.iter() {
            canvas.draw_dot(x + p.x, y + p.y);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clean(&mut self, dot: &Point) {
        self.values.remove(dot);
    }
}

impl PointSet for Sprite {
    fn get_point_set(&self) -> Option<&HashSet<Point>> {
        Some(&self.values)
    }
    fn get_max(&self) -> Point {
        self.max
    }
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Canvas for Sprite {
    fn draw_dot(&mut self, x: IndexType, y: IndexType) -> bool {
        self.values.insert((x, y).as_point());
        self.max = (max(x, self.max.x), max(y, self.max.y)).as_point();
        true
    }

    fn clean_dot(&mut self, x: IndexType, y: IndexType) -> bool {
        self.values.remove(&(x, y).as_point());
        true
    }
}
