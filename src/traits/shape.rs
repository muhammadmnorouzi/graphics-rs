use crate::traits::canvas::Canvas;

pub trait Shape {
    fn draw_to(&mut self, drawable: &mut impl Canvas);
}
