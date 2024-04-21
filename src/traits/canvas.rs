use super::shape::Shape;
use crate::color::Color;

pub trait Canvas {
    fn draw_shape(&mut self, shape: &mut impl Shape);
    fn change_color(&mut self, color: Color);
    fn clamp_row(&self, row: i64) -> i64;
    fn clamp_col(&self, col: i64) -> i64;
    fn set_pixel(&mut self, row: i64, col: i64);
    fn set_pixel_color(&mut self, row: i64, col: i64, color: Color);
    fn color_at(&self, index: usize) -> Color;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn fits_inside(&self, row: i64, col: i64) -> bool;
    fn fill(&mut self);
    fn antialiasing(&self) -> bool;
    fn resolution(&self) -> usize;
    fn color(&self) -> Color;
    fn buffer_mut_slice(&mut self) -> &mut [Color];
}
