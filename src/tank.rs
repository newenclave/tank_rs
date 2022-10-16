use std::{time::Duration, collections::HashSet};

use crate::{
    animation_builder::AnimationBuilder,
    canvas::Canvas,
    drawable::Drawable,
    game_object::game_object::GameObject,
    obstacle::{Obstacles, Obstacle},
    position::{AsPoint, IndexType, Point},
    shot::Shot,
    sprite::Sprite,
    timer::Timer, 
    direction::Direction, 
    point_set::PointSet, 
    game_object::{GameObjectArea, GameObjectAnimated}
};

pub struct Tank {
    area: GameObjectAnimated,
    border: Sprite,
    direction: Direction,
    shots: Vec<Shot>,
    recharge_delay: Timer,
}

const TANK_SPRITE: &'static str = r#"
+    *
|    *
|  *****
|**  *   *
|   *** **
|** * * **
|** ***  
|*      **
|   *** 
-
+    *
|    *
|  *****
|*   *  ** 
|** *** **
|** * *  
|   *** **
|**     **
|   *** 
-
+    *
|    *
|  *****
|**  *  ** 
|** ***  
|   * * **
|** *** **
|**      *
|   *** 
"#;

impl Tank {
    pub fn new(x: IndexType, y: IndexType) -> Self {
        let tank_animated = AnimationBuilder::new_static()
            .add_from_string(TANK_SPRITE)
            .build();
        let mut a = Sprite::new();
        for x in 0..=tank_animated.get_max().x {
            for y in 0..=tank_animated.get_max().y {
                a.draw_dot(x, y);
            }
        }
        Self {
            area: GameObjectAnimated::new(tank_animated, x, y),
            border: a,
            direction: Direction::Up,
            shots: Vec::new(),
            recharge_delay: Timer::new(Duration::from_millis(250)),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.area.sprite.update(delta);
        for s in self.shots.iter_mut() {
            s.update(delta);
        }
        self.shots.retain(|s| {
            !s.is_done()
        });
        self.recharge_delay.update(delta);
    }

    pub fn rotate_90(&mut self) {
        self.area.sprite.rotate_90();
        self.border.rotate_90();
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
        let mut turned = false;
        while self.direction != dir {
            turned = true;
            self.rotate_90();
        }
        if !turned {
            let fixed_pos = dir.go_forward(self.get_pos());
            self.area.move_to(fixed_pos.x, fixed_pos.y);
            self.area.sprite.update_force();    
        }
    }

    fn go_back(&mut self) {
        let fixed_pos = self.direction.go_back(self.get_pos());
        self.area.move_to(fixed_pos.x, fixed_pos.y);
    }

    pub fn shoot(&mut self) {
        if self.recharge_delay.ready() {
            let center = self.get_front_center();
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

    pub fn get_front_center(&self) -> Point {
        let pos = self.area.get_pos();
        let spos = (self.get_width() / 2, self.get_height() / 2);
        match self.direction {
            Direction::Up => (spos.0 + pos.x, pos.y).as_point(),
            Direction::Right => (spos.0 + pos.x, spos.1 + pos.y).as_point(),
            Direction::Down => (spos.0 + pos.x, spos.1 + pos.y).as_point(),
            Direction::Left => (pos.x, spos.1 + pos.y).as_point(),
        }
    }

    fn check_shots_obstacle(&mut self, o: &mut Obstacle) {
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
            self.check_shots_obstacle(o);
        }
    }
}

impl Drawable for Tank {
    fn draw(&self, canvas: &mut dyn crate::canvas::Canvas) {
        self.area.draw_to_canvas(canvas);
        for s in self.shots.iter() {
            s.draw(canvas);
        }
    }
}

impl GameObject for Tank {
    fn get_area(&self) -> &dyn GameObjectArea {
        &self.area
    }
    fn get_point_set(&self) -> Option<&HashSet<Point>> {
        self.border.get_point_set()
    }
}
