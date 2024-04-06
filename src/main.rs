mod image;

use crate::image::Image;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> std::io::Result<()> {
    let mut image = Image::create(WIDTH, HEIGHT, None);
    image.fill_rect(100, 100, 600, 600, 0x00FF00);
    image.fill_circle(WIDTH / 2, HEIGHT / 2, 200, 0xFF00F0);
    image.save("image.ppm")?;

    Ok(())
}
