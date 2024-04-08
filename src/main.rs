mod image;

use crate::image::Image;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> std::io::Result<()> {
    let mut image = Image::create(WIDTH, HEIGHT, None);
    image.set_color(0xFFFFFF);
    image.fill();
    image.set_color(0x00FF00);
    image.fill_rect(100, 100, 600, 600);
    image.set_color(0xFF00F0);
    image.fill_circle(WIDTH / 2, HEIGHT / 2, 200);
    image.set_color(0xF0000F);
    image.draw_line(0, 0, WIDTH, HEIGHT);
    image.draw_triangle(10, 10, 0, 400, 20, 450);
    image.save("image.ppm")?;

    Ok(())
}
