mod image;
mod rect;

use rect::Rect;

use crate::image::Image;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> std::io::Result<()> {
    let mut image = Image::create(WIDTH, HEIGHT, None);
    image.fill_rect(&Rect { x: 100, y: 100, w: 600, h: 600}, 0x00FF00);
    image.save("image.ppm")?;

    Ok(())
}
