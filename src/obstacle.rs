use std::{collections::HashSet, time::Duration};

use crate::{
    canvas::Canvas,
    drawable::Drawable,
    game_object::game_object::GameObject,
    position::{AsPoint, IndexType, Point},
    sprite::Sprite, game_object_area::GameObjectArea, game_object_impls::GameObjectStatic,
};

pub struct Obstacle {
    area: GameObjectStatic,
    transparent: bool,
    solid: bool,
    visible: bool,
    ground: bool,
    invisible: HashSet<Point>,
}

impl Obstacle {
    pub fn new_frame(x1: IndexType, y1: IndexType, x2: IndexType, y2: IndexType) -> Self {
        let mut s = Sprite::new();
        s.draw_rectangle((x1, y1).as_point(), (x2, y2).as_point());
        Self {
            area: GameObjectStatic::new(s, x1, y1),
            transparent: false,
            solid: true,
            visible: true,
            ground: false,
            invisible: HashSet::new(),
        }
    }

    pub fn new_rect(x1: IndexType, y1: IndexType, x2: IndexType, y2: IndexType) -> Self {
        let mut s = Sprite::new();
        for x in 0..x2 - x1 {
            for y in 0..y2 - y1 {
                s.draw_dot(x, y);
            }
        }
        Self {
            area: GameObjectStatic::new(s, x1, y1),
            transparent: false,
            solid: true,
            visible: true,
            ground: false,
            invisible: HashSet::new(),
        }
    }

    pub fn new_transparent_rect(
        x1: IndexType,
        y1: IndexType,
        x2: IndexType,
        y2: IndexType,
        factor: IndexType,
    ) -> Self {
        let mut s = Sprite::new();
        for x in 0..x2 - x1 {
            for y in 0..y2 - y1 {
                if (x + y) % factor == 0 {
                    s.draw_dot(x, y);
                }
            }
        }
        Self {
            area: GameObjectStatic::new(s, x1, y1),
            transparent: true,
            solid: false,
            visible: true,
            ground: false,
            invisible: HashSet::new(),
        }
    }

    pub fn new_circle(x1: IndexType, y1: IndexType, radius: IndexType) -> Self {
        let mut s = Sprite::new();
        for r in 1..=radius {
            s.draw_circle((x1, y1).as_point(), r);
        }
        Self {
            area: GameObjectStatic::new(s, x1 - radius, y1 - radius),
            transparent: false,
            solid: true,
            visible: true,
            ground: false,
            invisible: HashSet::new(),
        }
    }

    pub fn set_transparent(&mut self, value: bool) {
        self.transparent = value;
    }

    pub fn set_ground(&mut self, value: bool) {
        self.ground = value;
    }

    pub fn set_solid(&mut self, value: bool) {
        self.solid = value;
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn update(&mut self, _: Duration) {
        self.invisible.clear();
    }

    pub fn is_done(&self) -> bool {
        self.area.sprite.is_empty()
    }

    pub fn clean(&mut self, dot: &Point) {
        self.area.sprite.clean(dot);
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

    pub fn is_ground(&self) -> bool {
        self.ground
    }

    pub fn set_invisivle_dots(&mut self, val: HashSet<Point>) {
        self.invisible = val;
    }
}

pub struct Obstacles {
    values: Vec<Obstacle>,
}

impl Obstacles {
    pub fn new() -> Self {
        Self { values: Vec::new() }
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
            self.area.draw_to_canvas(canvas);
            let area_pos = self.area.get_pos();
            for p in self.invisible.iter() {
                canvas.clean_dot(p.x + area_pos.x, p.y + area_pos.y);
            }
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
    fn get_area(&self) -> &dyn GameObjectArea {
        &self.area
    }
}
