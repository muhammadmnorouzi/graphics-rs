use crate::{
    color::IsColor,
    traits::{canvas::Canvas, shape::Shape},
};

pub struct Circle {
    cx: usize,
    cy: usize,
    radius: usize,
}

impl Circle {
    pub fn new(cx: usize, cy: usize, radius: usize) -> Self {
        Self { cx, cy, radius }
    }

    fn get_circle_rect_area(&self, canvas: &mut impl Canvas) -> (usize, usize, usize, usize) {
        let (center_x, center_y, radius) = (self.cx as f64, self.cy as f64, self.radius as f64);
        let x1 = canvas.clamp_col(center_x - radius) as usize;
        let x2 = canvas.clamp_col(center_x + radius) as usize;
        let y1 = canvas.clamp_row(center_y - radius) as usize;
        let y2 = canvas.clamp_row(center_y + radius) as usize;

        (x1, x2, y1, y2)
    }

    fn draw_antialiased(
        &mut self,
        canvas: &mut impl Canvas,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) {
        for row in y1 as i64..=y2 as i64 {
            for col in x1 as i64..=x2 as i64 {
                let resolution = canvas.resolution();

                let mut count: usize = 0;
                for i in 0..resolution as i64 {
                    for j in 0..resolution as i64 {
                        let res1 = (resolution + 1) as i64;
                        let dx = row * res1 * 2 + 2 + i * 2 - res1 * self.cx as i64 * 2 - res1;
                        let dy = col * res1 * 2 + 2 + j * 2 - res1 * self.cy as i64 * 2 - res1;

                        if dx.pow(2) + dy.pow(2) <= res1.pow(2) * (self.radius as i64).pow(2) * 4 {
                            count += 1;
                        }
                    }
                }

                let alpha = canvas.color().alpha() as f64 * count as f64 / resolution.pow(2) as f64;
                let color = canvas
                    .color()
                    .with_alpha(alpha.clamp(0f64, u8::MAX as f64) as u8); // ((color&0xFF000000)>>(3*8))*count/OLIVEC_AA_RES/OLIVEC_AA_RES;
                canvas.set_pixel_color(row as usize, col as usize, color);
            }
        }
    }

    fn draw_simple(
        &mut self,
        canvas: &mut impl Canvas,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) {
        for row in y1..=y2 {
            for col in x1..=x2 {
                let valid_pixel = row.abs_diff(self.cx).pow(2) + col.abs_diff(self.cy).pow(2)
                    <= self.radius.pow(2);

                if valid_pixel {
                    canvas.set_pixel(row, col);
                }
            }
        }
    }
}

impl Shape for Circle {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let (x1, x2, y1, y2) = self.get_circle_rect_area(canvas);

        if canvas.antialiasing() {
            self.draw_antialiased(canvas, x1, x2, y1, y2);
        } else {
            self.draw_simple(canvas, x1, x2, y1, y2);
        }
    }
}
