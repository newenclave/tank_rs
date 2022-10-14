use crate::{braille, canvas::Canvas, position::IndexType};

pub struct BrailleCanvas {
    area: Vec<Vec<u8>>,
}

impl BrailleCanvas {
    pub fn new(x: usize, y: usize) -> Self {
        let x_fix = x / 2 + x % 2;
        let y_fix = y / 4 + if y % 4 == 0 { 0 } else { 1 };
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
    fn draw_dot(&mut self, x: IndexType, y: IndexType) -> bool {
        if x >= 0 && y >= 0 {
            let (fix_x, fix_y, pos_x, pos_y) = self.to_coord(x as usize, y as usize);
            if fix_x < self.area.len() && fix_y < self.area[fix_x].len() {
                let value = self.area[fix_x][fix_y];
                self.area[fix_x][fix_y] = braille::set_dot(value, pos_x, pos_y);
                return true;
            }
        }
        false
    }
}
