use std::{fs::File, io::Write, num::NonZeroUsize};

use crate::{
    color::{Color, IsColor},
    traits::{
        canvas::Canvas, handles_draw_request::HandlesDrawRequest, requests_draw::RequestDraw,
        shape::Shape,
    },
};

pub struct SimpleCanvas<'a> {
    data: Vec<Color>,
    width: usize,
    height: usize,
    color: Color,
    antialiasing: bool,
    antialiasing_resolution: usize,
    draw_request_handler: Option<&'a dyn HandlesDrawRequest>,
}

impl<'a> SimpleCanvas<'a> {
    pub fn new(
        width: usize,
        height: usize,
        fill_color: Option<Color>,
        antialiasing: bool,
        antialiasing_resolution: usize,
    ) -> Self {
        let antialiasing_resolution = if antialiasing_resolution == 0 {
            1
        } else {
            antialiasing_resolution
        };
        let fill_color = fill_color.unwrap_or(0);

        Self {
            data: vec![fill_color; width * height],
            width,
            height,
            color: fill_color,
            antialiasing,
            draw_request_handler: None,
            antialiasing_resolution: antialiasing_resolution,
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

    fn antialiasing(&self) -> bool {
        self.antialiasing
    }

    fn resolution(&self) -> usize {
        self.antialiasing_resolution
    }

    fn color(&self) -> Color {
        self.color
    }

    fn set_pixel(&mut self, row: usize, col: usize) {
        self.set_pixel_color(row, col, self.color);
    }

    fn set_pixel_color(&mut self, row: usize, col: usize, color: Color) {
        if self.fits_inside(row, col) {
            let index = self.width * row + col;
            let old_color = self.data[index];
            self.data[index] = old_color.mix(color);
        }
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
