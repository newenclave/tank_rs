use std::{collections::HashSet, time::Duration};

use crate::{sprite::Sprite, position::{Point, AsPoint, IndexType}, canvas::Canvas, drawable::Drawable, game_object::GameObject};

pub struct Obstacle {
    sprite: Sprite,
    pos: Point,
    transparent: bool,
    solid: bool,
    visible: bool,
}

impl Obstacle {
    pub fn new_frame(x1: IndexType, y1: IndexType, x2: IndexType, y2: IndexType) -> Self {
        let mut s = Sprite::new();
        s.draw_rectangle((x1, y1).as_point(), (x2, y2).as_point());
        Self {
            sprite: s,
            pos: (x1, y1).as_point(),
            transparent: false,
            solid: true,
            visible: true,
        }
    }

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
            solid: true,
            visible: true,
        }
    }

    pub fn new_transparent_rect(x1: IndexType, y1: IndexType, x2: IndexType, y2: IndexType) -> Self {
        let mut s = Sprite::new();
        for x in 0..x2-x1 {
            for y in 0..y2-y1 {
                if (x + y) % 3 == 0 {
                    s.draw_dot(x, y);
                } 
            }
        } 
        Self {
            sprite: s,
            pos: (x1, y1).as_point(),
            transparent: true,
            solid: false,
            visible: true,
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
            solid: true,
            visible: true,
        }
    }

    pub fn set_transparent(&mut self, value: bool) {
        self.transparent = value;
    }

    pub fn set_solid(&mut self, value: bool) {
        self.solid = value;
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn update(&mut self, _: Duration) {}

    pub fn is_done(&self) -> bool {
        self.sprite.is_empty()
    }

    pub fn clean(&mut self, dot: &Point) {
        self.sprite.clean(dot);
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    } 

    pub fn is_transparent(&self) -> bool {
        self.transparent
    } 

    pub fn is_visible(&self) -> bool {
        self.visible
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
        if self.visible {
            self.sprite.draw_to_canvas(canvas, self.pos.x, self.pos.y);
        }
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
        if self.transparent {
            None
        } else {
            self.sprite.get_point_set()
        }
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
