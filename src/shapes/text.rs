use crate::{
    traits::{canvas::Canvas, is_color::IsColor, shape::Shape},
    types::types::A,
};

pub struct Text {
    text: &'static str,
    size: usize,
    x: i64,
    y: i64,
}

impl Text {
    pub fn new(text: &'static str, size: usize, x: i64, y: i64) -> Self {
        Self { text, size, x, y }
    }
}

impl Shape for Text {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        for row in 0..A.len() {
            for col in 0..A[row].len() {
                canvas.change_color(canvas.color().with_alpha(A[row][col]));
                canvas.set_pixel(row + self.x as usize, col + self.y as usize)
            }
        }
    }
}
