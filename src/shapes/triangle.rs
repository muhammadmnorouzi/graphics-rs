use crate::{
    math::num_utils::NumUtils,
    traits::{canvas::Canvas, shape::Shape},
};

pub struct Triangle {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x3: usize,
    y3: usize,
}

impl Triangle {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize, x3: usize, y3: usize) -> Self {
        Self {
            x1,
            x2,
            y1,
            y2,
            x3,
            y3,
        }
    }
}

impl Shape for Triangle {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let (mut x1, mut y1, mut x2, mut y2, mut x3, mut y3) = (
            self.x1 as f64,
            self.y1 as f64,
            self.x2 as f64,
            self.y2 as f64,
            self.x3 as f64,
            self.y3 as f64,
        );

        NumUtils::order_triangle_vertices_by_y(
            &mut x1, &mut y1, &mut x2, &mut y2, &mut x3, &mut y3,
        );

        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let dx13 = x3 - x1;
        let dy13 = y3 - y1;

        for row in y1 as i64..=y2 as i64 {
            if row >= 0 && row < canvas.height() as i64 {
                let mut s1 = if dy12 != 0f64 {
                    (row as f64 - y1) * dx12 / dy12 + x1
                } else {
                    x1
                };
                let mut s2 = if dy13 != 0f64 {
                    (row as f64 - y1) * dx13 / dy13 + x1
                } else {
                    x1
                };
                if s1 > s2 {
                    std::mem::swap(&mut s1, &mut s2)
                }
                for col in s1 as i64..=s2 as i64 {
                    if col >= 0 && col < canvas.width() as i64 {
                        canvas.set_pixel(row, col);
                    }
                }
            }
        }

        let dx32 = x2 - x3;
        let dy32 = y2 - y3;
        let dx31 = x1 - x3;
        let dy31 = y1 - y3;

        for row in y2 as i64..=y3 as i64 {
            if row >= 0 && row < canvas.height() as i64 {
                let mut s1 = if dy32 != 0f64 {
                    (row as f64 - y3) * dx32 / dy32 + x3
                } else {
                    x3
                };
                let mut s2 = if dy31 != 0f64 {
                    (row as f64 - y3) * dx31 / dy31 + x3
                } else {
                    x3
                };
                if s1 > s2 {
                    std::mem::swap(&mut s1, &mut s2)
                }
                for col in s1 as i64..=s2 as i64 {
                    if col >= 0 && col < canvas.width() as i64 {
                        canvas.set_pixel(row, col);
                    }
                }
            }
        }
    }
}
