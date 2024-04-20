use crate::traits::{canvas::Canvas, shape::Shape};

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
        let x1 = canvas.clamp_col(self.x1 as i64);
        let x2 = canvas.clamp_col(self.x2 as i64);
        let y1 = canvas.clamp_row(self.y1 as i64);
        let y2 = canvas.clamp_row(self.y2 as i64);

        let (x1, x2, y1, y2) = if x2 < x1 {
            (x2, x2, y2, y1)
        } else {
            (x1, x2, y1, y2)
        };

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0 {
            for row in y1..y2 {
                if canvas.fits_inside(row, x1) {
                    canvas.set_pixel(row as usize, x1 as usize);
                }
            }
        } else {
            let slope = dy as f64 / dx as f64;

            for col in x1..x2 {
                let y = col as f64 * slope + y1 as f64;
                let row = y as i64;

                if canvas.fits_inside(row, col) {
                    canvas.set_pixel(row as usize, col as usize);
                }
            }
        }
    }
}
