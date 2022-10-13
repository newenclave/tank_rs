use std::{collections::HashSet, cmp::{max, min}};
use crate::position::{Point, IndexType, AsPoint};

type Rect = (Point, Point);
type PointSet = HashSet<Point>;

fn max_x(pos1: &Point, pos2: &Point) -> IndexType {
    max(pos1.x, pos2.x)
} 

fn min_x(pos1: &Point, pos2: &Point) -> IndexType {
    min(pos1.x, pos2.x)
} 

fn max_y(pos1: &Point, pos2: &Point) -> IndexType {
    max(pos1.y, pos2.y)
} 

fn min_y(pos1: &Point, pos2: &Point) -> IndexType {
    min(pos1.y, pos2.y)
} 

/*
  (0.x, 0.y)
            +----+
            |    |
            +----+
                  (1.x, 1.y)
 */

fn game_object_overlap(rect1: &Rect, rect2: &Rect) -> bool {
    // checks is rects are empty.
    if rect1.0.x == rect1.1.x || rect1.0.y == rect1.1.y 
       || rect2.0.x == rect2.1.x || rect2.0.y == rect2.1.y {
        return false;
    }

    if rect1.0.x > rect2.1.x || rect2.0.x > rect1.1.x {
        return false;
    }

    if rect1.0.y > rect2.1.y || rect2.0.y > rect1.1.y {
        return false;
    }
    true
}

fn game_object_intersection(obj1: &(&Rect, &PointSet), obj2: &(&Rect, &PointSet)) -> Vec<(Point, Point)> {
    let mut res: Vec<(Point, Point)> = Vec::new();
    if game_object_overlap(&obj1.0, &obj2.0) {
        let r1 = obj1.0;
        let r2 = obj2.0;
        let h1 = obj1.1;
        let h2 = obj2.1;
        
        for x in max_x(&r1.0, &r2.0)..=min_x(&r1.1, &r2.1) {
            for y in max_y(&r1.0, &r2.0)..=min_y(&r1.1, &r2.1) {
                let point1 = (x - r1.0.x, y - r1.0.y).as_point();
                let point2 = (x - r2.0.x, y - r2.0.y).as_point();
                if h1.contains(&point1) && h2.contains(&point2) {
                    res.push((point1, point2));
                }
            }
        }
    }
    res
}

pub trait GameObject {
    fn get_point_set(&self) -> Option<&HashSet<Point>>;
    fn get_position(&self) -> Point;
    fn get_width(&self) -> IndexType;
    fn get_height(&self) -> IndexType;
    fn get_rect(&self) -> (Point, Point) {
        let pos = self.get_position();
        (pos, Point::new(pos.x + self.get_width(), pos.y + self.get_height()))    
    }

    fn get_overlap(&self, other: &dyn GameObject) -> Vec<(Point, Point)> {
        if let Some(my_set) = self.get_point_set() {
            if let Some(other_set) = other.get_point_set() {
                let my_rect = self.get_rect();
                let other_rect = other.get_rect();
                return game_object_intersection(&(&my_rect, my_set), &(&other_rect, other_set));
            }
        }
        Vec::new()
    }
}
