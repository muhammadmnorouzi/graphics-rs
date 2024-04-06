use crate::rect::Rect;
use std::{fs::File, io::Write, vec};

pub struct Image {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn create(width: usize, height: usize, fill_color: Option<u32>) -> Self {
        let fill_color = fill_color.unwrap_or(0);

        Self {
            data: vec![fill_color; width * height],
            width,
            height,
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        File::write(
            &mut file,
            format!("P6\n{} {} 255\n", self.width, self.height).as_bytes(),
        )?;

        let mut bytes: [u8; 3] = [0, 0, 0];
        for pixel in &self.data {
            bytes[0] = (pixel >> (8 * 0) & 0xFF) as u8;
            bytes[1] = (pixel >> (8 * 1) & 0xFF) as u8;
            bytes[2] = (pixel >> (8 * 2) & 0xFF) as u8;

            File::write(&mut file, &bytes)?;
        }

        Ok(())
    }

    pub fn fill_rect(&mut self, rect: &Rect, fill_color: u32) {
        let x = rect.x.clamp(0, self.width);
        let w = (x + rect.w).clamp(0, self.width);
        let y = rect.y.clamp(0, self.height);
        let h = (y + rect.h).clamp(0, self.height);

        for col in y..=h {
            for row in x..=w {
                self.data[row * self.width + col] = fill_color
            }
        }
    }

    pub fn fill_circle(
        &mut self,
        center_x: usize,
        center_y: usize,
        radius: usize,
        fill_color: u32,
    ) {
        let (x1, x2, y1, y2) = self.get_circle_rect_area(center_x, center_y, radius);

        for col in y1..=y2 {
            for row in x1..=x2 {
                let valid_distance =
                    row.abs_diff(center_x).pow(2) + col.abs_diff(center_y).pow(2) <= radius.pow(2);

                if valid_distance {
                    self.data[row * self.width + col] = fill_color;
                }
            }
        }
    }

    fn get_circle_rect_area(
        &self,
        center_x: usize,
        center_y: usize,
        radius: usize,
    ) -> (usize, usize, usize, usize) {
        let x1 = if center_x >= radius {
            center_x - radius
        } else {
            0
        };
        let x2 = if center_x + radius > self.width {
            self.width
        } else {
            center_x + radius
        };
        let y1 = if center_y < radius {
            0
        } else {
            center_y - radius
        };
        let y2 = if center_y + radius < self.height {
            center_y + radius
        } else {
            self.height
        };

        (x1, x2, y1, y2)
    }
}
