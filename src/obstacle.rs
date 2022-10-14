use std::{collections::HashSet, time::Duration};

use crate::{sprite::Sprite, position::{Point, AsPoint, IndexType}, canvas::Canvas, drawable::Drawable, game_object::GameObject};

pub struct Obstacle {
    sprite: Sprite,
    pos: Point,
    transparent: bool,
    solid: bool
}

impl Obstacle {
    pub fn new_rect(x1: IndexType, y1: IndexType, x2: IndexType, y2: IndexType) -> Self {
        let mut s = Sprite::new();
        for x in 0..x2-x1 {
            for y in 0..y2-y1 {
                s.draw_dot(x, y);
            }
        } 
        Self {
            sprite: s,
            pos: (x1, y1).as_point(),
            transparent: false,
            solid: true
        }
    }
    pub fn new_circle(x1: IndexType, y1: IndexType, radius: IndexType) -> Self {
        let mut s = Sprite::new();
        for r in 1..=radius {
            s.draw_circle((x1, y1).as_point(), r);
        } 
        Self {
            sprite: s,
            pos: (x1 - radius, y1 - radius).as_point(),
            transparent: false,
            solid: true
        }
    }

    pub fn update(&mut self, _: Duration) {}

    pub fn is_done(&self) -> bool {
        self.sprite.is_empty()
    }

    pub fn clean(&mut self, dot: &Point) {
        self.sprite.clean(dot);
    }

}

pub struct Obstacles {
    values: Vec<Obstacle>
}

impl Obstacles {
    pub fn new() -> Self {
        Self {
            values: Vec::new()
        }
    }

    pub fn add_obstacle(&mut self, o: Obstacle) {
        self.values.push(o);
    }

    pub fn update(&mut self, delta: Duration) {
        for o in self.values.iter_mut() {
            o.update(delta);
        }
        self.values.retain(|o| !o.is_done());
    }

    pub fn get_all(&self) -> &Vec<Obstacle> {
        &self.values
    }

    pub fn get_all_mut(&mut self) -> &mut Vec<Obstacle> {
        &mut self.values
    }

}

impl Drawable for Obstacle {
    fn draw(&self, canvas: &mut dyn Canvas) {
        self.sprite.draw_to_canvas(canvas, self.pos.x, self.pos.y);
    }
}

impl Drawable for Obstacles {
    fn draw(&self, canvas: &mut dyn Canvas) {
        for o in self.values.iter() {
            o.draw(canvas);
        }
    }
}

impl GameObject for Obstacle {
    fn get_point_set(&self) -> Option<&HashSet<Point>> {
        self.sprite.get_point_set()
    }

    fn get_position(&self) -> Point {
        self.pos
    }

    fn get_width(&self) -> IndexType {
        self.sprite.get_width()
    }

    fn get_height(&self) -> IndexType {
        self.sprite.get_height()
    }
}
