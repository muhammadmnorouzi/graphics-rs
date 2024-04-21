use crate::{
    data::ascii_letters::{F as B}, traits::{canvas::Canvas, is_color::IsColor, shape::Shape}
};

use super::rect::Rect;

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
        let original_color = canvas.color();

        for row in 0..B.len() {
            for col in 0..B[row].len() {
                canvas.change_color(canvas.color().with_alpha(B[row][col]));

                let row = row as i64 * self.size as i64 + self.y;
                let col = col as i64 * self.size as i64 + self.x;

                canvas.draw_shape(&mut Rect::new(col, row, self.size, self.size));
                canvas.set_pixel(row + self.x, col + self.y);
            }
        }

        canvas.change_color(original_color);
    }
}
