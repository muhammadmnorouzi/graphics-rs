use crate::color::Color;
use super::shape::Shape;

pub trait Canvas {
    fn draw_shape(&mut self, shape :&mut impl Shape);
    fn change_color(&mut self, color: Color);
    fn clamp_row(&self, row: f64) -> f64;
    fn clamp_col(&self, col: f64) -> f64;
    fn set_pixel(&mut self, row: usize, col: usize);
}