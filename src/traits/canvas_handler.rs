use super::canvas::Canvas;

pub trait CanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T);
}
