use crate::{
    color::Color,
    traits::{canvas::Canvas, shape::Shape},
};

pub struct Circle {
    cx: usize,
    cy: usize,
    radius: usize,
    color: Color,
}

impl Circle {
    pub fn new(cx: usize, cy: usize, radius: usize, color: Color) -> Self {
        Self {
            cx,
            cy,
            radius,
            color,
        }
    }

    fn get_circle_rect_area(&self, canvas: &mut impl Canvas) -> (usize, usize, usize, usize) {
        let (center_x, center_y, radius) = (self.cx as f64, self.cy as f64, self.radius as f64);
        let x1 = canvas.clamp_col(center_x - radius) as usize;
        let x2 = canvas.clamp_col(center_x + radius) as usize;
        let y1 = canvas.clamp_row(center_y - radius) as usize;
        let y2 = canvas.clamp_row(center_y + radius) as usize;

        (x1, x2, y1, y2)
    }
}

impl Shape for Circle {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        canvas.change_color(self.color);
        let (x1, x2, y1, y2) = self.get_circle_rect_area(canvas);

        for row in y1..=y2 {
            for col in x1..=x2 {
                let valid_distance = col.abs_diff(self.cx).pow(2) + row.abs_diff(self.cy).pow(2)
                    <= self.radius.pow(2);

                if valid_distance {
                    canvas.set_pixel(row, col);
                }
            }
        }
    }
}
