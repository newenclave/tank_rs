use crate::{canvas::Canvas, position::IndexType};

pub trait Drawable {
    fn draw(&self, canvas: &mut dyn Canvas);
}
