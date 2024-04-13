use std::{fs::File, io::Write, num::NonZeroUsize};

use crate::{
    color::{Color, IsColor},
    math::num_utils::NumUtils,
    traits::{
        antialiasable::Antialiasable, canvas::Canvas, handles_draw_request::HandlesDrawRequest,
        requests_draw::RequestDraw, shape::Shape,
    },
};

pub struct SimpleCanvas<'a> {
    data: Vec<Color>,
    width: usize,
    height: usize,
    color: Color,
    antialiasing: bool,
    antialiasing_resolution: Option<NonZeroUsize>,
    draw_request_handler: Option<&'a dyn HandlesDrawRequest>,
}

impl<'a> SimpleCanvas<'a> {
    pub fn new(
        width: usize,
        height: usize,
        fill_color: Option<Color>,
        antialiasing: bool,
        anti_aliasing_resolution: Option<NonZeroUsize>,
    ) -> Self {
        let fill_color = fill_color.unwrap_or(0);

        Self {
            data: vec![fill_color; width * height],
            width,
            height,
            color: fill_color,
            antialiasing,
            antialiasing_resolution: anti_aliasing_resolution,
            draw_request_handler: None,
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        File::write(
            &mut file,
            format!("P6\n{} {} 255\n", self.width, self.height).as_bytes(),
        )?;

        for pixel in &self.data {
            File::write(&mut file, &[pixel.red(), pixel.green(), pixel.blue()])?;
        }

        Ok(())
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let x = self.clamp_col(x as f64) as usize;
        let w = self.clamp_col(x as f64 + width as f64) as usize;
        let y = self.clamp_row(y as f64) as usize;
        let h = self.clamp_row(y as f64 + height as f64) as usize;

        for row in y..h {
            for col in x..w {
                self.set_pixel(row, col);
            }
        }
    }

    pub fn draw_triangle(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        x3: usize,
        y3: usize,
    ) {
        let (mut x1, mut y1, mut x2, mut y2, mut x3, mut y3) = (
            x1 as f64, y1 as f64, x2 as f64, y2 as f64, x3 as f64, y3 as f64,
        );

        NumUtils::order_triangle_vertices_by_y(
            &mut x1, &mut y1, &mut x2, &mut y2, &mut x3, &mut y3,
        );

        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let dx13 = x3 - x1;
        let dy13 = y3 - y1;

        for row in y1 as i64..=y2 as i64 {
            if row >= 0 && row < self.height as i64 {
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
                    if col >= 0 && col < self.width as i64 {
                        self.set_pixel(row as usize, col as usize);
                    }
                }
            }
        }

        let dx32 = x2 - x3;
        let dy32 = y2 - y3;
        let dx31 = x1 - x3;
        let dy31 = y1 - y3;

        for row in y2 as i64..=y3 as i64 {
            if row >= 0 && row < self.height as i64 {
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
                    if col >= 0 && col < self.width as i64 {
                        self.set_pixel(row as usize, col as usize);
                    }
                }
            }
        }
    }
}

impl<'a> Canvas for SimpleCanvas<'a> {
    fn draw_shape(&mut self, shape: &mut impl Shape) {
        shape.draw_to(self);
    }

    fn change_color(&mut self, color: Color) {
        self.color = color
    }

    fn clamp_row(&self, row: f64) -> f64 {
        row.clamp(0f64, (self.height - 1) as f64)
    }

    fn clamp_col(&self, col: f64) -> f64 {
        col.clamp(0f64, (self.width - 1) as f64)
    }

    fn set_pixel(&mut self, row: usize, col: usize) {
        let index = self.width * row + col;
        let old_color = self.data[index];
        self.data[index] = old_color.mix(self.color);
    }

    fn color_at(&self, index: usize) -> Color {
        self.data[index]
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn fits_inside(&self, row: usize, col: usize) -> bool {
        return row < self.height && col < self.width;
    }

    fn fill(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = self.color;
        }
    }
}

impl<'a> Antialiasable for SimpleCanvas<'a> {
    fn antialiasing_enabled(&self) -> bool {
        self.antialiasing
    }

    fn antialiasing_resolution(&self) -> NonZeroUsize {
        self.antialiasing_resolution
            .unwrap_or(NonZeroUsize::new(1).unwrap())
    }
}

impl<'a> RequestDraw<'a> for SimpleCanvas<'a> {
    fn set_draw_request_handler<T: HandlesDrawRequest>(&mut self, handler: &'a T) {
        self.draw_request_handler = Some(handler)
    }

    fn request_draw(&self) {
        if let Some(draw_handler) = self.draw_request_handler {
            draw_handler.draw();
        }
    }
}
