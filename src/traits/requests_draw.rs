use super::handles_draw_request::HandlesDrawRequest;

pub trait RequestDraw<'a> {
    fn set_draw_request_handler<T: HandlesDrawRequest>(&mut self, handler: &'a T);
    fn request_draw(&self);
}
