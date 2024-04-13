pub trait HandlesDrawRequest: Sync {
    fn draw(&self);
}
