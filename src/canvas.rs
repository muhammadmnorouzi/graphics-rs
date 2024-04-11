use crate::color::{Color, IsColor};
use std::{fs::File, io::Write, vec};

pub struct Canvas {
    data: Vec<Color>,
    width: usize,
    height: usize,
    color: Color,
}

impl Canvas {
    pub fn clamp_row(&self, row: f64) -> f64 {
        row.clamp(0f64, (self.height - 1) as f64)
    }

    pub fn clamp_col(&self, col: f64) -> f64 {
        col.clamp(0f64, (self.width - 1) as f64)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color_at(&self, index: usize) -> Color {
        self.data[index]
    }

    fn set_color_at(&mut self, row: usize, col: usize) {
        let index = self.width * row + col;
        let old_color = self.data[index];
        self.data[index] = old_color.mix(self.color);
    }

    pub fn create(width: usize, height: usize, fill_color: Option<Color>) -> Self {
        let fill_color = fill_color.unwrap_or(0);

        Self {
            data: vec![fill_color; width * height],
            width,
            height,
            color: fill_color,
        }
    }

    pub fn change_color(&mut self, color: u32) {
        self.color = color
    }

    pub fn fill(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = self.color;
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
                self.set_color_at(row, col);
            }
        }
    }

    pub fn fill_circle(&mut self, center_x: usize, center_y: usize, radius: usize) {
        let (x1, x2, y1, y2) = self.get_circle_rect_area(center_x, center_y, radius);

        for row in y1..=y2 {
            for col in x1..=x2 {
                let valid_distance =
                    col.abs_diff(center_x).pow(2) + row.abs_diff(center_y).pow(2) <= radius.pow(2);

                if valid_distance {
                    self.set_color_at(row, col);
                }
            }
        }
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let x1 = self.clamp_col(x1 as f64) as usize;
        let x2 = self.clamp_col(x2 as f64) as usize;
        let y1 = self.clamp_row(y1 as f64) as usize;
        let y2 = self.clamp_row(y2 as f64) as usize;

        let (x1, x2) = Canvas::order_asc(x1, x2);
        let (y1, y2) = Canvas::order_asc(y1, y2);

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0 {
            for row in y1..y2 {
                self.set_color_at(row, x1);
            }
        } else {
            let slope = dy as f64 / dx as f64;

            for col in x1..x2 {
                let y = col as f64 * slope + y1 as f64;
                let row = y as usize;
                self.set_color_at(row, col);
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
        Canvas::order_triangle_vertices_by_y(&mut x1, &mut y1, &mut x2, &mut y2, &mut x3, &mut y3);

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
                        self.set_color_at(row as usize, col as usize);
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
                        self.set_color_at(row as usize, col as usize);
                    }
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
        let (center_x, center_y, radius) = (center_x as f64, center_y as f64, radius as f64);
        let x1 = self.clamp_col((center_x - radius)) as usize;
        let x2 = self.clamp_col((center_x + radius)) as usize;
        let y1 = self.clamp_row((center_y - radius)) as usize;
        let y2 = self.clamp_row((center_y + radius)) as usize;

        (x1, x2, y1, y2)
    }

    fn order_asc(a: usize, b: usize) -> (usize, usize) {
        (usize::min(a, b), usize::max(a, b))
    }

    fn order_triangle_vertices_by_y(
        x1: &mut f64,
        y1: &mut f64,
        x2: &mut f64,
        y2: &mut f64,
        x3: &mut f64,
        y3: &mut f64,
    ) {
        if *y1 > *y2 {
            std::mem::swap(x1, x2);
            std::mem::swap(y1, y2);
        }

        if *y2 > *y3 {
            std::mem::swap(x2, x3);
            std::mem::swap(y2, y3);
        }

        if *y1 > *y2 {
            std::mem::swap(x1, x2);
            std::mem::swap(y1, y2);
        }
    }
}