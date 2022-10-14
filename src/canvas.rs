use std::cmp::{max, min};

use crate::position::{AsPoint, IndexType, Point};

pub trait Canvas {
    fn draw_dot(&mut self, x: IndexType, y: IndexType) -> bool;
    fn clean_dot(&mut self, x: IndexType, y: IndexType) -> bool;

    fn draw_line(&mut self, from: Point, to: Point) {
        let dx: i32 = to.x as i32 - from.x as i32;
        let dy: i32 = to.y as i32 - from.y as i32;
        if dx > 0 || dy > 0 {
            if dx.abs() > dy.abs() {
                let mut y: f32 = from.y as f32 + 0.5;
                let dly = dy as f32 / dx as f32;
                if dx > 0 {
                    for ix in from.x..=to.x {
                        self.draw_dot(ix, y.floor() as IndexType);
                        y = y + dly;
                    }
                } else {
                    for ix in (from.x..=to.x).rev() {
                        self.draw_dot(ix, y.floor() as IndexType);
                        y = y - dly;
                    }
                }
            } else {
                // dy >= dx
                let mut x: f32 = from.x as f32 + 0.5;
                let dlx = dx as f32 / dy as f32;
                if dy > 0 {
                    for iy in from.y..=to.y {
                        self.draw_dot(x.floor() as IndexType, iy);
                        x = x + dlx;
                    }
                } else {
                    for iy in (from.y..=to.y).rev() {
                        self.draw_dot(x.floor() as IndexType, iy);
                        x = x - dlx;
                    }
                }
            }
        }
    }

    fn draw_circle(&mut self, center: Point, radius: IndexType) {
        //
        //if center.x >= radius && center.y >= radius && radius > 0 {
        for x in 0..=radius {
            for y in 0..=radius {
                let xi = center.x - radius + x;
                let yi = center.y - radius + y;
                let distance = center.distance_as_f32(&(xi, yi).as_point());
                let delta = (distance - radius as f32).abs();
                if delta < 0.4 {
                    self.draw_dot(xi, yi);
                    self.draw_dot(center.x + radius - x, yi);
                    self.draw_dot(xi, center.y + radius - y);
                    self.draw_dot(center.x + radius - x, center.y + radius - y);
                } else if distance < radius as f32 {
                    break;
                }
            }
        }
    }

    fn draw_rectangle(&mut self, from: Point, to: Point) {
        let (min_x, min_y) = (min(from.x, to.x), min(from.y, to.y));
        let (max_x, max_y) = (max(from.x, to.x), max(from.y, to.y));
        for i in min_x..=max_x {
            self.draw_dot(i, min_y);
            self.draw_dot(i, max_y);
        }
        for i in min_y..=max_y {
            self.draw_dot(min_x, i);
            self.draw_dot(max_x, i);
        }
    }

    /*
        ' ' - increments X
        '\r' - ignored
        '\n' - increments y, resets x
        '|' - resets X
        '+' - resets X AND y
        '-' - ends the sprite
        any - plots dot, incremets x
        example:
        +  *
        | * *
        |*****
    */
    fn draw_from_string<'a>(&mut self, value: &'a str) -> &'a str {
        let mut x: IndexType = 0;
        let mut y: IndexType = 0;
        let mut id = 0;
        for (i, c) in value.chars().enumerate() {
            match c {
                ' ' => {
                    x += 1;
                }
                '+' => {
                    y = 0;
                    x = 0;
                }
                '\r' => {}
                '\n' => {
                    x = 0;
                    y += 1;
                }
                '|' => {
                    x = 0;
                }
                '-' => {
                    id = i;
                    break;
                }
                _ => {
                    self.draw_dot(x, y);
                    x += 1;
                }
            }
            id = i;
        }
        &value[id + 1..value.len()]
    }
}
