use crate::traits::{canvas::Canvas, shape::Shape};

pub struct Rect {
    x: i64,
    y: i64,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Shape for Rect {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let x = canvas.clamp_col(self.x) as usize;
        let w = canvas.clamp_col(self.x + self.width as i64) as usize;
        let y = canvas.clamp_row(self.y) as usize;
        let h = canvas.clamp_row(self.y + self.height as i64) as usize;

        for row in y..h {
            for col in x..w {
                canvas.set_pixel(row as i64, col as i64);
            }
        }
    }
}
