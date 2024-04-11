use std::{num::NonZeroU32, rc::Rc};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::canvas::Canvas;

pub trait GraphicsHandler {
    fn draw(&mut self, canvas: &mut Canvas);
}

pub struct Graphics {
    canvas: Canvas,
}

impl Graphics {
    pub fn create(canvas: Canvas) -> Self {
        Self { canvas }
    }

    pub fn run(&mut self, handler: &mut impl GraphicsHandler) -> Result<(), String> {
        let event_loop = EventLoop::new().map_err(|error| error.to_string())?;

        let window = Rc::new(
            WindowBuilder::new()
                .with_resizable(false)
                .with_inner_size(PhysicalSize::new(
                    self.canvas.width() as u32,
                    self.canvas.height() as u32,
                ))
                .build(&event_loop)
                .map_err(|error| error.to_string())?,
        );

        let graphics_context =
            softbuffer::Context::new(window.clone()).map_err(|error| error.to_string())?;

        let mut surface = softbuffer::Surface::new(&graphics_context, window.clone())
            .map_err(|error| error.to_string())?;

        surface
            .resize(
                NonZeroU32::new(self.canvas.width() as u32).unwrap(),
                NonZeroU32::new(self.canvas.height() as u32).unwrap(),
            )
            .expect("could not resize surface!");

        event_loop
            .run(move |event, window_target| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        handler.draw(&mut self.canvas);

                        let mut buffer = surface.buffer_mut().expect("could not get mut buffer!");
                        for index in 0..self.canvas.width() * self.canvas.height() {
                            buffer[index as usize] = self.canvas.color_at(index);
                        }

                        buffer.present().unwrap();
                    }
                    _ => (),
                },
                _ => {
                    window.request_redraw()
                },
            })
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}
