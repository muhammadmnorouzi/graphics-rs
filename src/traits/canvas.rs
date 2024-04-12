use super::shape::Shape;
use crate::color::Color;

pub trait Canvas {
    fn draw_shape(&mut self, shape: &mut impl Shape);
    fn change_color(&mut self, color: Color);
    fn clamp_row(&self, row: f64) -> f64;
    fn clamp_col(&self, col: f64) -> f64;
    fn set_pixel(&mut self, row: usize, col: usize);
    fn color_at(&self, index: usize) -> Color;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn fits_inside(&self, row: usize, col: usize) -> bool;
}
