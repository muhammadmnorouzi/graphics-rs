mod color;
mod canvas;

use std::{num::NonZeroU32, rc::Rc};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::canvas::Canvas;

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let mut canvas = Canvas::create(WIDTH as usize, HEIGHT as usize, None);
    canvas.set_color(color::BLACK);
    canvas.fill();
    canvas.set_color(color::GREEN);
    canvas.fill_rect(100, 100, 600, 600);
    canvas.set_color(color::RED);
    canvas.fill_circle(WIDTH as usize / 2, HEIGHT as usize / 2, 200);
    canvas.set_color(color::BLUE);
    canvas.draw_line(0, 0, WIDTH as usize, HEIGHT as usize);
    canvas.set_color(color::WHITE);
    canvas.draw_triangle(10, 10, 0, 400, 20, 450);
    canvas.save("image.ppm").map_err(|error| error.to_string())?;
    show(&canvas)?;

    Ok(())
}

fn show(canvas: &Canvas) -> Result<(), String> {
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
                    for index in 0..canvas.width() * canvas.height() {
                        buffer[index as usize] = canvas.color_at(index);
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
