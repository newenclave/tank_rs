use crate::{
    position::{
        Point, 
        IndexType, AsPoint
    }, 
    point_set::PointSet, 
    canvas::Canvas
};

pub trait GameObjectArea {
    fn get_point_set(&self) -> Option<&std::collections::HashSet<Point>>;
    fn get_pos(&self) -> Point;
    fn get_width(&self) -> crate::position::IndexType;
    fn get_height(&self) -> crate::position::IndexType;
    fn get_center_pos(&self) -> Point;
    fn move_to(&mut self, x: IndexType, y: IndexType);
    fn move_center_to(&mut self, x: IndexType, y: IndexType);
    fn draw_to_canvas(&self, canvas: &mut dyn Canvas);
}

pub struct GameObjectAreaImpl<T> {
    pub sprite: T,
    pos: Point,
}

impl<T> GameObjectAreaImpl<T> {
    pub fn new(point_set: T, x: IndexType, y: IndexType) -> Self {
        Self {
            sprite: point_set,
            pos: (x, y).as_point(), 
        }
    }

    pub fn set_point_set(&mut self, ps: T) {
        self.sprite = ps;
    }
}

impl<T> GameObjectArea for GameObjectAreaImpl<T> 
where 
    T: PointSet
{
    fn get_point_set(&self) -> Option<&std::collections::HashSet<Point>> {
        self.sprite.get_point_set()
    }

    fn get_pos(&self) -> Point {
        self.pos
    }

    fn get_center_pos(&self) -> Point {
        let p = self.get_pos();
        (p.x + self.get_width() / 2, p.y + self.get_height() / 2).as_point()
    }

    fn get_width(&self) -> crate::position::IndexType {
        self.sprite.get_max().x + 1
    }

    fn get_height(&self) -> crate::position::IndexType {
        self.sprite.get_max().y + 1
    }

    fn move_to(&mut self, x: IndexType, y: IndexType) {
        self.pos.move_to(x, y);
    }

    fn move_center_to(&mut self, x: IndexType, y: IndexType) {
        self.move_to(x - self.get_width() / 2, y - self.get_height() / 2)
    }

    fn draw_to_canvas(&self, canvas: &mut dyn Canvas) {
        if let Some(points) = self.get_point_set() {
            for p in points.iter() {
                canvas.draw_dot(self.pos.x + p.x, self.pos.y + p.y);
            }
        }
    }

}
