use crate::{canvas::Canvas, position::IndexType, position::Point, sprite::Sprite, timer::Timer, point_set::PointSet};
use std::{cmp::max, collections::HashSet, time::Duration};

pub struct Animated {
    sprites: Vec<Sprite>,
    max_pos: Point,
    id: usize,
    delay: Timer,
    looped: bool,
}

impl Animated {
    pub fn new(switch_delay: Duration) -> Self {
        Self {
            sprites: Vec::new(),
            max_pos: Point::new(0, 0),
            id: 0,
            delay: Timer::new(switch_delay),
            looped: false,
        }
    }

    pub fn new_static() -> Self {
        Self {
            sprites: Vec::new(),
            max_pos: Point::new(0, 0),
            id: 0,
            delay: Timer::new(Duration::MAX),
            looped: false,
        }
    }

    pub fn new_looped(switch_delay: Duration) -> Self {
        Self {
            sprites: Vec::new(),
            max_pos: Point::new(0, 0),
            id: 0,
            delay: Timer::new(switch_delay),
            looped: true,
        }
    }

    pub fn add_from_string(&mut self, mut s: &str) {
        while !s.is_empty() {
            let mut next = Sprite::new();
            s = next.draw_from_string(s);
            self.add_sprite(next);
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.max_pos.x = max(sprite.get_max().x, self.max_pos.x);
        self.max_pos.y = max(sprite.get_max().y, self.max_pos.y);
        self.sprites.push(sprite);
    }

    pub fn update(&mut self, delta: Duration) {
        if self.sprites.is_empty() || self.is_static() {
            return;
        }
        if self.delay.update(delta) {
            if !self.is_done() {
                self.id = (self.id + 1) % self.sprites.len();
                self.delay.reset();
            }
        }
    }

    pub fn update_force(&mut self) {
        if !self.sprites.is_empty() {
            self.id = (self.id + 1) % self.sprites.len();
            self.delay.reset();
        }
    }

    fn is_static(&self) -> bool {
        self.delay.is_max_duration()
    }

    fn end(&self) -> bool {
        self.sprites.is_empty() || self.id == (self.sprites.len() - 1)
    }

    fn is_done(&self) -> bool {
        !self.looped && self.end() && self.delay.ready()
    }

    pub fn reset(&mut self) {
        if self.is_done() {
            self.delay.reset();
            self.id = 0;
        }
    }

    pub fn get_current_sprite(&self) -> Option<&Sprite> {
        if self.sprites.is_empty() {
            None
        } else {
            Some(&self.sprites[self.id])
        }
    }

    pub fn rotate_90(&mut self) {
        for sprite in self.sprites.iter_mut() {
            sprite.rotate_90();
        }
    }

    pub fn draw_to_canvas(&self, canvas: &mut dyn Canvas, x: IndexType, y: IndexType) {
        if let Some(sprite) = self.get_current_sprite() {
            sprite.draw_to_canvas(canvas, x, y);
        }
    }

    pub fn is_empty(&self) -> bool {
        for s in self.sprites.iter() {
            if !s.is_empty() {
                return false;
            }
        }
        true
    }
}


impl PointSet for Animated {
    fn get_point_set(&self) -> Option<&HashSet<Point>> {
        if let Some(sprite) = self.get_current_sprite() {
            sprite.get_point_set()
        } else {
            None
        }
    }
    
    fn get_max(&self) -> Point {
        if self.sprites.is_empty() {
            Point::new(0, 0)
        } else {
            self.sprites[self.id].get_max()
        }
    }

    fn is_empty(&self) -> bool {
        if self.sprites.is_empty() {
            return true;
        }
        for s in self.sprites.iter() {
            if s.is_empty() {
                return true;
            }
        }
        false
    }
}
