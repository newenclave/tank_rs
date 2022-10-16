use std::collections::HashSet;
use crate::position::Point;

pub trait PointSet {
    fn get_point_set(&self) -> Option<&HashSet<Point>>;
    fn get_max(&self) -> Point;
    fn is_empty(&self) -> bool;
}
