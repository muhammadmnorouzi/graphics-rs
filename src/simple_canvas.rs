use std::{fs::File, io::Write};

use crate::{
    color::{self, Color, IsColor},
    traits::{
        canvas::Canvas, handles_draw_request::HandlesDrawRequest, requests_draw::RequestDraw,
        shape::Shape,
    },
};

pub struct SimpleCanvas<'a> {
    buffer: Vec<Color>,
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
        let antialiasing_resolution = antialiasing_resolution.clamp(1, usize::MAX);
        let fill_color = fill_color.unwrap_or(color::BLACK);

        Self {
            buffer: vec![fill_color; width * height],
            width,
            height,
            color: fill_color,
            antialiasing,
            draw_request_handler: None,
            antialiasing_resolution: antialiasing_resolution,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn save(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        File::write(
            &mut file,
            format!("P6\n{} {} 255\n", self.width, self.height).as_bytes(),
        )?;

        for i in 0..self.size() {
            let pixel = self.color_at(i);
            let red = pixel.red();
            let green = pixel.green();
            let blue = pixel.blue();

            File::write(&mut file, &[red, green, blue])?;
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

    fn clamp_row(&self, row: i64) -> i64 {
        row.clamp(0, (self.height - 1) as i64)
    }

    fn clamp_col(&self, col: i64) -> i64 {
        col.clamp(0, (self.width - 1) as i64)
    }

    fn color_at(&self, index: usize) -> Color {
        return self.buffer[index];
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn fits_inside(&self, row: i64, col: i64) -> bool {
        return row < self.height as i64 && col < self.width as i64;
    }

    fn fill(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.set_pixel_color(row, col, self.color);
            }
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
        if self.fits_inside(row as i64, col as i64) {
            let index = self.width * row + col;
            let old_color = self.color_at(index);

            self.buffer[index] = old_color.mix(color);
        }
    }
    
    fn buffer_mut_slice(&mut self) -> &mut [Color] {
        self.buffer.as_mut_slice()
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
