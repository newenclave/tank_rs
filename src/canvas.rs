use std::cmp::{min, max};

use crate::position::{IndexType, Point, ToPoint};

fn distance_to(lhs: &Point, rhs: &Point) -> f32 {
    let delta_x = lhs.x as f32 - rhs.x as f32;
    let delta_y = lhs.y as f32 - rhs.y as f32;
   (delta_x * delta_x + delta_y * delta_y).sqrt()
}

pub trait Canvas {
    fn draw_dot(&mut self, x: IndexType, y: IndexType) -> bool;

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
            } else { // dy >= dx
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
        if center.x >= radius && center.y >= radius && radius > 0 {
            for x in 0..=radius {
                for y in 0..=radius {
                    let xi = center.x - radius + x;
                    let yi = center.y - radius + y;
                    let distance = distance_to(&center, &(xi, yi).as_point());
                    let delta = (distance - radius as f32).abs();
                    if  delta < 0.4 {
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

}
