use std::{time::Duration, collections::HashSet};

use crate::{
    animated::Animated,
    canvas::Canvas,
    drawable::Drawable,
    position::{AsPoint, IndexType, Point},
    sprite::Sprite,
    timer::Timer, game_object::GameObject,
};

const BULLET_SPRITE: &'static str = r#"x
  |X
  |x
"#;

const BULLET_EXPLODE_01: &'static str = r#"XX
  |XX
"#;

const BULLET_EXPLODE_02: &'static str = r#" X 
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
   + X X
   |X X X
   | X X
"#;

const BILLET_ANIMATED: [&'static str; 6] = [
    BULLET_EXPLODE_01,
    BULLET_EXPLODE_02,
    BULLET_EXPLODE_03,
    BULLET_EXPLODE_04,
    BULLET_EXPLODE_05,
    BULLET_EXPLODE_06,
];

pub struct Shot {
    sprite: Animated,
    pos: Point,
    explode_pos: Point,
    direction: u8,
    delay: Timer,
    exploding: bool,
}

impl Shot {
    pub fn new(x: IndexType, y: IndexType, dir: u8) -> Self {
        let mut s = Animated::new_static();
        s.add_sprite(Sprite::new_from_string(BULLET_SPRITE));
        if dir == 1 || dir == 3 {
            s.rotate_90();
        }
        Self {
            sprite: s,
            pos: (x, y).as_point(),
            explode_pos: (x, y).as_point(),
            direction: dir,
            delay: Timer::new(Duration::from_millis(25)),
            exploding: false,
        }
    }

    pub fn forward(&mut self) {
        if !self.exploding {
            match self.direction {
                0 => self.pos.y -= 1,
                1 => self.pos.x += 1,
                2 => self.pos.y += 1,
                3 => self.pos.x -= 1,
                _ => {}
            }
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.sprite.update(delta);
        if self.delay.update(delta) {
            self.forward()
        }
        if self.exploding {
            self.fix_explode_pos();
        }
      }

    pub fn explode(&mut self) -> bool {
        if !self.exploding {
            self.explode_pos = Point {
                x: self.pos.x + self.sprite.get_current_width() / 2,
                y: self.pos.y + self.sprite.get_current_height() / 2,
            };
            self.exploding = true;
            self.delay = Timer::from_millis(500);
            self.sprite = Animated::new_looped(Duration::from_millis(100));
            for s in BILLET_ANIMATED {
                self.sprite.add_sprite(Sprite::new_from_string(s));
            }
            self.fix_explode_pos();
            return true;
        }
        false
    }

    fn fix_explode_pos(&mut self) {
        let cur_w = self.sprite.get_current_width() / 2;
        let cur_h = self.sprite.get_current_height() / 2;
        self.pos = (self.explode_pos.x - cur_w, self.explode_pos.y - cur_h).as_point();
    }

    pub fn is_done(&self) -> bool {
        self.exploding && self.delay.ready()
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }
}

impl Drawable for Shot {
    fn draw(&self, canvas: &mut dyn Canvas) {
        self.sprite.draw_to_canvas(canvas, self.pos.x, self.pos.y)
    }
}

impl GameObject for Shot {
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