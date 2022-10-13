use crate::{braille, canvas::Canvas};

pub struct BrailleCanvas {
    area: Vec<Vec<u8>>,
}

impl BrailleCanvas {
    pub fn new(x: usize, y: usize) -> Self {
        let x_fix = x / 2 + x % 2;
        let y_fix = y / 4 + if y % 4 == 0 {0} else {1};
        let e_val = braille::empty();
        let mut canvas = Vec::with_capacity(x_fix);
        for _ in 0..x_fix {
            let mut row = Vec::with_capacity(y_fix);
            for _ in 0..y_fix {
                row.push(e_val);
            }
            canvas.push(row);
        }
        Self { area: canvas }
    }

    fn to_coord(&self, x: usize, y: usize) -> (usize, usize, usize, usize) {
        let fixed_x = x / 2;
        let fixed_y = y / 4;
        (fixed_x, fixed_y, x % 2, y % 4)
    }

    pub fn area(&self) -> &Vec<Vec<u8>> {
        &self.area
    }
    
}
 
impl Canvas for BrailleCanvas {
    fn get_width(&self) -> usize {
        self.area.len() * 2
    }

    fn get_height(&self) -> usize {
        if !self.area.is_empty() {
            self.area[0].len() * 4
        } else {
            0
        }
    }

    fn draw_dot(&mut self, x: usize, y: usize) -> bool {
        let (fix_x, fix_y, pos_x, pos_y) = self.to_coord(x, y);
        if fix_x < self.area.len() && fix_y < self.area[fix_x].len() {
            let value = self.area[fix_x][fix_y];
            self.area[fix_x][fix_y] = braille::set_dot(value, pos_x, pos_y);
            return true;
        } 
        false
    }
}
