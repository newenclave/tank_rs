use std::{time::Duration, collections::HashSet};

use crate::{
    animated::Animated,
    drawable::Drawable,
    position::{AsPoint, IndexType, Point},
    shot::Shot,
    sprite::Sprite, game_object::GameObject, obstacle::Obstacles,
};

pub struct Tank {
    sprite: Animated,
    direction: u8,
    pos: Point,
    shots: Vec<Shot>,
}

const TANK_SPRITE_01: &'static str = r#"
+   *
|   *
| *****
|*  *   
|  *** *
|* * * *
|* ***  
|      *
|  *** 
"#;
const TANK_SPRITE_02: &'static str = r#"
+   *
|   *
| *****
|   *  * 
|* *** *
|* * * 
|  *** *
|*     *
|  *** 
"#;
const TANK_SPRITE_03: &'static str = r#"
+   *
|   *
| *****
|*  *  * 
|* ***  
|  * * *
|* *** *
|*    
|  *** 
"#;

impl Tank {
    pub fn new(x: IndexType, y: IndexType) -> Self {
        let mut tank_animated = Animated::new_static();
        tank_animated.add_sprite(Sprite::new_from_string(TANK_SPRITE_01));
        tank_animated.add_sprite(Sprite::new_from_string(TANK_SPRITE_02));
        tank_animated.add_sprite(Sprite::new_from_string(TANK_SPRITE_03));
        Self {
            sprite: tank_animated,
            direction: 0,
            pos: (x, y).as_point(),
            shots: Vec::new(),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.sprite.update(delta);
        for s in self.shots.iter_mut() {
            s.update(delta);
        }
        self.shots.retain(|s| s.get_pos().x >= 0 && s.get_pos().x <= 120
                                  && s.get_pos().y >= 0 && s.get_pos().y <= 80
                                  && !s.is_done());
    }

    pub fn rotate_90(&mut self) {
        self.sprite.rotate_90();
        self.direction = (self.direction + 1) % 4;
    }

    pub fn rotate_270(&mut self) {
        self.rotate_90();
        self.rotate_90();
        self.rotate_90();
    }

    pub fn go_left(&mut self) {
        while self.direction != 3 {
            self.rotate_90();
        }
        self.pos.x -= 1;
        self.sprite.force_update();
    }

    pub fn go_right(&mut self) {
        while self.direction != 1 {
            self.rotate_90();
        }
        self.pos.x += 1;
        self.sprite.force_update();
    }

    pub fn go_up(&mut self) {
        while self.direction != 0 {
            self.rotate_90();
        }
        self.pos.y -= 1;
        self.sprite.force_update();
    }

    pub fn go_down(&mut self) {
        while self.direction != 2 {
            self.rotate_90();
        }
        self.pos.y += 1;
        self.sprite.force_update();
    }

    fn go_back(&mut self) {
        match self.direction {
            0 => { self.pos.y += 1; },
            1 => { self.pos.x -= 1; },
            2 => { self.pos.y -= 1; },
            3 => { self.pos.x += 1; },
            _ => {},
        }
    }

    fn go_forward(&mut self) {
        match self.direction {
            0 => { self.pos.y -= 1; },
            1 => { self.pos.x += 1; },
            2 => { self.pos.y += 1; },
            3 => { self.pos.x -= 1; },
            _ => {},
        }
    }

    pub fn shoot(&mut self) {
        let center = self.get_center();
        self.shots
            .push(Shot::new(center.x, center.y, self.direction));
    }

    pub fn explode(&mut self) {
        if !self.shots.is_empty() {
            self.shots[0].explode();
        }
    }

    pub fn get_center(&self) -> Point {
        let pos = (
            self.sprite.get_current_width() / 2,
            self.sprite.get_current_height() / 2,
        )
            .as_point();
        match self.direction {
            0 => (self.pos.x + pos.x, self.pos.y).as_point(),
            1 => (self.pos.x + pos.x, self.pos.y + pos.y).as_point(),
            2 => (self.pos.x + pos.x, self.pos.y + pos.y).as_point(),
            3 => (self.pos.x, self.pos.y + pos.y).as_point(),
            _ => self.pos,
        }
    }

    pub fn check_obstacles(&mut self, obstacles: &mut Obstacles) {
        for o in obstacles.get_all_mut().iter_mut() {
            let tank_overlap = self.get_overlap(o);
            if !tank_overlap.is_empty() {
                if tank_overlap[0].0.x < self.get_height() / 2 {
                    self.go_back();
                } else {
                    self.go_forward();
                }
            }
            for b in self.shots.iter_mut() {
                let bullet_overlap = b.get_overlap(o);
                if !bullet_overlap.is_empty() {
                    b.explode();
                }
                for v in bullet_overlap {
                    o.clean(&v.1);
                }
            }
        }
    }

}

impl Drawable for Tank {
    fn draw(&self, canvas: &mut dyn crate::canvas::Canvas) {
        if let Some(sprite) = self.sprite.get_current_sprite() {
            sprite.draw_to_canvas(canvas, self.pos.x, self.pos.y);
        }
        for s in self.shots.iter() {
            s.draw(canvas);
        }
    }
}

impl GameObject for Tank {
    fn get_point_set(&self) -> Option<&HashSet<Point>> {
        self.sprite.get_point_set()
    }

    fn get_position(&self) -> Point {
        self.pos
    }

    fn get_width(&self) -> IndexType {
        self.sprite.get_current_width()
    }

    fn get_height(&self) -> IndexType {
        self.sprite.get_current_height()
    }
}

