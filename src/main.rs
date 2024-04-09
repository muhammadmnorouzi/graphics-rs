mod color;
mod image;

use std::{num::NonZeroU32, rc::Rc};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::image::Image;

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let mut image = Image::create(WIDTH as usize, HEIGHT as usize, None);
    image.set_color(color::BLACK);
    image.fill();
    image.set_color(color::GREEN);
    image.fill_rect(100, 100, 600, 600);
    image.set_color(color::RED);
    image.fill_circle(WIDTH as usize / 2, HEIGHT as usize / 2, 200);
    image.set_color(color::BLUE);
    image.draw_line(0, 0, WIDTH as usize, HEIGHT as usize);
    image.set_color(color::WHITE);
    image.draw_triangle(10, 10, 0, 400, 20, 450);
    image.save("image.ppm").map_err(|error| error.to_string())?;
    show(&image)?;

    Ok(())
}

fn show(image: &Image) -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|error| error.to_string())?;

    let window = Rc::new(
        WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
            .build(&event_loop)
            .map_err(|error| error.to_string())?,
    );

    let graphics_context =
        softbuffer::Context::new(window.clone()).map_err(|error| error.to_string())?;

    let mut surface = softbuffer::Surface::new(&graphics_context, window.clone())
        .map_err(|error| error.to_string())?;

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                WindowEvent::RedrawRequested => {
                    surface
                        .resize(
                            NonZeroU32::new(WIDTH).unwrap(),
                            NonZeroU32::new(HEIGHT).unwrap(),
                        )
                        .expect("could not resize surface!");

                    let mut buffer = surface.buffer_mut().expect("could not get mut buffer!");
                    for index in 0..image.width() * image.height() {
                        buffer[index as usize] = image.color_at(index);
                    }

                    buffer.present().unwrap();
                }
                _ => (),
            },
            _ => (),
        })
        .map_err(|error| error.to_string())?;

    Ok(())
}
