mod canvas;
mod color;

use std::{num::NonZeroU32, ops::ControlFlow, rc::Rc};

use color::{Color, IsColor};
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
    canvas.change_color(color::BLACK);
    canvas.fill();
    canvas.change_color(Color::create(u8::MAX, 0, 0, u8::MAX));
    canvas.fill_rect(0, 0, 800, 600);
    canvas.change_color(Color::create(0, 0, u8::MAX, 128));
    canvas.fill_rect(0, 200, 800, 800);
    canvas.change_color(Color::create(u8::MAX, u8::MAX, u8::MAX, u8::MAX));
    canvas.fill_rect(300, 0, 200, 800);
    canvas.change_color(Color::create(0, 255, 0, 80));
    canvas.fill_circle(400  , 400, 400);
    canvas
        .save("image.ppm")
        .map_err(|error| error.to_string())?;
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
