use crate::{
    math::num_utils::NumUtils,
    traits::{canvas::Canvas, shape::Shape},
};

pub struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self { x1, x2, y1, y2 }
    }
}

impl Shape for Line {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let x1 = canvas.clamp_col(self.x1 as f64) as usize;
        let x2 = canvas.clamp_col(self.x2 as f64) as usize;
        let y1 = canvas.clamp_row(self.y1 as f64) as usize;
        let y2 = canvas.clamp_row(self.y2 as f64) as usize;

        let (x1, x2) = NumUtils::order_asc(x1, x2);
        let (y1, y2) = NumUtils::order_asc(y1, y2);

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0 {
            for row in y1..y2 {
                canvas.set_pixel(row, x1);
            }
        } else {
            let slope = dy as f64 / dx as f64;

            for col in x1..x2 {
                let y = col as f64 * slope + y1 as f64;
                let row = y as usize;

                if canvas.fits_inside(row, col) {
                    canvas.set_pixel(row, col);
                }
            }
        }
    }
}
