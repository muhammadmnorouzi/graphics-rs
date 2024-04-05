use std::{fs::File, io::Write, vec};

pub struct Image {
    data: Vec<u32>,
    width: usize,
    height: usize
}

impl Image{
    pub fn create(width: usize, height: usize, fill_color: Option<u32>) -> Self {
        let fill_color = fill_color.unwrap_or(0);

        Self {
            data: vec![fill_color; width * height],
            width,
            height
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        File::write(&mut file , format!("P6\n{} {} 255\n", self.width , self.height).as_bytes())?;
        
        let mut bytes: [u8; 3] = [0,0,0];
        for pixel in &self.data {
            bytes[0] = (pixel >> (8 * 0) & 0xFF) as u8;
            bytes[1] = (pixel >> (8 * 1) & 0xFF) as u8;
            bytes[2] = (pixel >> (8 * 2) & 0xFF) as u8;

            File::write(&mut file , &bytes)?;
        }

        Ok(())
    }
}

