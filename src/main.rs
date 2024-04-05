mod image;

use crate::image::Image;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> std::io::Result<()> {
    let image = Image::create(WIDTH, HEIGHT, None);
    image.save("image.ppm")?;

    Ok(())
}
