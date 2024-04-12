use std::num::NonZeroUsize;

pub trait Antialiasable {
    fn antialiasing_enabled(&self) -> bool;
    fn antialiasing_resolution(&self) -> NonZeroUsize;
}
