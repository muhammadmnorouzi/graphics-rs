use crate::traits::{canvas::Canvas, shape::Shape};

pub struct Text {
    text: &'static str,
    size: usize,
}

impl Text {
    pub fn new(text: &'static str, size: usize) -> Self {
        Self { text, size }
    }
}

impl Shape for Text {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        for ch in self.text.chars() {
            print!("{}", ch);
        }
    }
}
