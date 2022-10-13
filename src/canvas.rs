pub trait Canvas {
    fn get_width(&self) -> usize; 
    fn get_height(&self) -> usize;
    fn draw_dot(&mut self, x: usize, y: usize) -> bool;
}
