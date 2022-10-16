use std::time::Duration;

use crate::{
    animated::Animated,
    canvas::Canvas,
    drawable::Drawable,
    game_object::GameObject,
    position::{AsPoint, IndexType, Point},
    sprite::Sprite,
    timer::Timer, direction::Direction, game_object_area::GameObjectArea, game_object_impls::GameObjectAnimated,
};

const BULLET_SPRITE: &'static str = r#"
  +x
  |X
  |x
"#;

const BULLET_EXPLODE_01: &'static str = r#"
  +XX
  |XX
"#;

const BULLET_EXPLODE_02: &'static str = r#" 
  + X 
  |X X
  | X
"#;

const BULLET_EXPLODE_03: &'static str = r#" 
  + XX 
  |X  X
  |X  X
  | XX
"#;

const BULLET_EXPLODE_04: &'static str = r#"  
  +  X 
  |X   X
  |X X X
  |X   X
  |  X
"#;

const BULLET_EXPLODE_05: &'static str = r#"   
  +   X 
  | X   X
  |X  X  X
  | X   X
  |   X
"#;

const BULLET_EXPLODE_06: &'static str = r#"   
  +    X 
  |  X   X
  | X  X  X
  |X X X X X
  | X  X  X
  |  X   X
  |    X
"#;

const BULLET_EXPLODE_07: &'static str = r#"  
   + X X
   |X X X
   | X X
"#;

const BULLET_ANIMATED: [&'static str; 7] = [
    BULLET_EXPLODE_01,
    BULLET_EXPLODE_02,
    BULLET_EXPLODE_03,
    BULLET_EXPLODE_04,
    BULLET_EXPLODE_05,
    BULLET_EXPLODE_06,
    BULLET_EXPLODE_07,
];

pub struct Shot {
    area: GameObjectAnimated,
    explode_pos: Point,
    direction: Direction,
    delay: Timer,
    exploding: bool,
}

impl Shot {
    pub fn new(x: IndexType, y: IndexType, dir: Direction) -> Self {
        let mut s = Animated::new_static();
        s.add_sprite(Sprite::new_from_string(BULLET_SPRITE));
        if dir == Direction::Left || dir == Direction::Right {
            s.rotate_90();
        }
        Self {
            area: GameObjectAnimated::new(s, x, y),
            explode_pos: (x, y).as_point(),
            direction: dir,
            delay: Timer::new(Duration::from_millis(25)),
            exploding: false,
        }
    }

    pub fn forward(&mut self) {
        if !self.exploding {
            let fixed_pos = self.direction.go_forward(self.area.get_pos());
            self.area.move_to(fixed_pos.x, fixed_pos.y);
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.area.sprite.update(delta);
        if self.delay.update(delta) {
            self.forward()
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
            let mut explode = Animated::new_looped(Duration::from_millis(100));
            for s in BULLET_ANIMATED {
                explode.add_sprite(Sprite::new_from_string(s));
            }
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
