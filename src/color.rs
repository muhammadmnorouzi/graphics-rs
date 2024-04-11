pub type Color = u32;

pub const BLACK: u32 = 0xFF000000;
pub const WHITE: u32 = 0xFFFFFFFF;
pub const RED: u32 = 0xFFFF0000;
pub const GREEN: u32 = 0xFF00FF00;
pub const BLUE: u32 = 0xFF0000FF;

pub trait IsColor {
    fn create(red: u8, green: u8, blue: u8, alpha: u8) -> Self;
    fn alpha(&self) -> u8;
    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
    fn mix(&self, rhs: Self) -> Self;
}

impl IsColor for Color {
    fn alpha(&self) -> u8 {
        ((self >> (8 * 3)) & 0xFF) as u8
    }

    fn red(&self) -> u8 {
        ((self >> (8 * 2)) & 0xFF) as u8
    }

    fn green(&self) -> u8 {
        ((self >> (8 * 1)) & 0xFF) as u8
    }

    fn blue(&self) -> u8 {
        ((self >> (8 * 0)) & 0xFF) as u8
    }

    fn mix(&self, rhs: Self) -> Self {
        let r1 = self.red() as f64;
        let g1 = self.green() as f64;
        let b1 = self.blue() as f64;
        let a1 = self.alpha() as f64;

        let r2 = rhs.red() as f64;
        let g2 = rhs.green() as f64;
        let b2 = rhs.blue() as f64;
        let a2 = rhs.alpha() as f64;
        
        let max = u8::MAX as f64;

        let red =   ((r1 * (max - a2) + r2 * a2) / max).clamp(0f64, max) as u8; 
        let green = ((g1 * (max - a2) + g2 * a2) / max).clamp(0f64, max) as u8; 
        let blue =  ((b1 * (max - a2) + b2 * a2) / max).clamp(0f64, max) as u8; 

        Color::create(red, green, blue, a1 as u8)
    }

    fn create(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let mut color = 0u32;

        color += (alpha as u32) << (8 * 3);
        color += (red as u32) << (8 * 2);
        color += (green as u32) << (8 * 1);
        color += (blue as u32) << (8 * 0);

        color
    }
}
