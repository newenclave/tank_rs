use std::time::Duration;

use crate::{
    animation_builder::AnimationBuilder,
    canvas::Canvas,
    drawable::Drawable,
    position::{AsPoint, IndexType, Point},
    timer::Timer, direction::Direction, 
    game_object::{GameObjectArea, GameObjectAnimated, GameObject},
};

const BULLET_SPRITE: &'static str = r#"
  +x
  |X
  |x
"#;

const BULLET_EXPLODE: &'static str = r#"
  +XX
  |XX
-  
  + X 
  |X X
  | X
-
  + XX 
  |X  X
  |X  X
  | XX
-  
  +  X 
  |X   X
  |X X X
  |X   X
  |  X
-
 +   X 
 | X   X
 |X  X  X
 | X   X
 |   X
-
+    X 
|  X   X
| X  X  X
|X X X X X
| X  X  X
|  X   X
|    X
-
   + X X
   |X X X
   | X X
"#;

pub struct Shot {
    area: GameObjectAnimated,
    explode_pos: Point,
    direction: Direction,
    delay: Timer,
    exploding: bool,
}

impl Shot {
    pub fn new(x: IndexType, y: IndexType, dir: Direction) -> Self {
        let s = AnimationBuilder::new_static()
                .add_sprite_from_string(BULLET_SPRITE)
                .modify(|mut a| {
                    if dir == Direction::Left || dir == Direction::Right {
                        a.rotate_90();
                    }
                    a
                })
                .build();
        Self {
            area: GameObjectAnimated::new(s, x, y),
            explode_pos: (x, y).as_point(),
            direction: dir,
            delay: Timer::new(Duration::from_millis(15)),
            exploding: false,
        }
    }

    fn forward(&mut self) {
        let fixed_pos = self.direction.go_forward(self.area.get_pos());
        self.area.move_to(fixed_pos.x, fixed_pos.y);
    }

    pub fn update(&mut self, delta: Duration) {
        self.area.sprite.update(delta);
        if self.delay.update(delta) {
            if !self.exploding {
                self.forward();
                self.delay.reset();
            }
        }
        if self.exploding {
            self.fix_explode_pos();
        } 
    }

    pub fn explode(&mut self) -> bool {
        if !self.exploding {
            self.explode_pos = self.area.get_center_pos();
            self.exploding = true;
            self.delay = Timer::from_millis(500);
            let explode = AnimationBuilder::new_looped(Duration::from_millis(100))
                .add_from_string(BULLET_EXPLODE)
                .build();
            self.area.set_point_set(explode); 
            self.fix_explode_pos();
            return true;
        }
        false
    }

    fn fix_explode_pos(&mut self) {
        self.area.move_center_to(self.explode_pos.x, self.explode_pos.y)
    }

    pub fn is_done(&self) -> bool {
        self.exploding && self.delay.ready()
    }

}

impl Drawable for Shot {
    fn draw(&self, canvas: &mut dyn Canvas) {
        self.area.draw_to_canvas(canvas)
    }
}

impl GameObject for Shot {
    fn get_area(&self) -> &dyn GameObjectArea {
        &self.area
    }
}
