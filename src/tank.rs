use std::{collections::HashSet, time::Duration};

use crate::{
    animated::Animated,
    canvas::Canvas,
    drawable::Drawable,
    game_object::GameObject,
    obstacle::Obstacles,
    position::{AsPoint, IndexType, Point},
    shot::Shot,
    sprite::Sprite,
    timer::Timer, 
    direction::Direction,
};

pub struct Tank {
    sprite: Animated,
    area: Sprite,
    direction: Direction,
    pos: Point,
    shots: Vec<Shot>,
    recharge_delay: Timer,
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
        let mut a = Sprite::new();
        for x in 0..tank_animated.get_current_width() {
            for y in 0..tank_animated.get_current_height() {
                a.draw_dot(x, y);
            }
        }
        Self {
            sprite: tank_animated,
            area: a,
            direction: Direction::Up,
            pos: (x, y).as_point(),
            shots: Vec::new(),
            recharge_delay: Timer::new(Duration::from_millis(250)),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.sprite.update(delta);
        for s in self.shots.iter_mut() {
            s.update(delta);
        }
        self.shots.retain(|s| {
            s.get_pos().x >= 0
                && s.get_pos().x <= 120
                && s.get_pos().y >= 0
                && s.get_pos().y <= 80
                && !s.is_done()
        });
        self.recharge_delay.update(delta);
    }

    pub fn rotate_90(&mut self) {
        self.sprite.rotate_90();
        self.area.rotate_90();
        self.direction = self.direction.next_clockwise();
    }

    pub fn go_left(&mut self) {
        self.goto(Direction::Left);
    }

    pub fn go_right(&mut self) {
        self.goto(Direction::Right);
    }

    pub fn go_up(&mut self) {
        self.goto(Direction::Up);
    }

    pub fn go_down(&mut self) {
        self.goto(Direction::Down);
    }

    fn goto(&mut self, dir: Direction) { 
        while self.direction != dir {
            self.rotate_90();
        }
        self.pos = dir.go_forward(self.pos);
        self.sprite.force_update();
    }

    fn go_back(&mut self) {
        self.pos = self.direction.go_back(self.pos)
    }

    pub fn shoot(&mut self) {
        if self.recharge_delay.ready() {
            let center = self.get_center();
            self.shots
                .push(Shot::new(center.x, center.y, self.direction));
            self.recharge_delay.reset()
        }
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
            Direction::Up => (self.pos.x + pos.x, self.pos.y).as_point(),
            Direction::Right => (self.pos.x + pos.x, self.pos.y + pos.y).as_point(),
            Direction::Down => (self.pos.x + pos.x, self.pos.y + pos.y).as_point(),
            Direction::Left => (self.pos.x, self.pos.y + pos.y).as_point(),
        }
    }

    pub fn check_obstacles(&mut self, obstacles: &mut Obstacles) {
        for o in obstacles.get_all_mut().iter_mut() {
            let tank_overlap = self.get_overlap(o);
            if !tank_overlap.is_empty() {
                if !o.is_transparent() {
                    self.go_back();
                } else {
                    if o.is_ground() {
                        o.set_invisivle_dots(tank_overlap.iter().map(|p| p.1).collect());
                    }
                }
            }
            for b in self.shots.iter_mut() {
                let bullet_overlap = b.get_overlap(o);
                if !o.is_transparent() {
                    if !bullet_overlap.is_empty() {
                        b.explode();
                    }
                    if !o.is_solid() {
                        for v in bullet_overlap {
                            o.clean(&v.1);
                        }
                    }
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
        self.area.get_point_set()
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
