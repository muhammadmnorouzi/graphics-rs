use std::num::NonZeroU32;
use std::sync::Arc;

use winit::window::Window;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::traits::canvas::Canvas;
use crate::traits::canvas_handler::CanvasHandler;
use crate::traits::{handles_draw_request::HandlesDrawRequest, requests_draw::RequestDraw};

pub struct Graphics<'a, T>
where
    T: RequestDraw<'a> + Canvas,
{
    canvas: &'a mut T,
    window: Option<Arc<Window>>,
}

impl<'a, T: RequestDraw<'a> + Canvas> Graphics<'a, T> {
    pub fn create(canvas: &'a mut T) -> Result<Self, String> {
        Ok(Self {
            canvas: canvas,
            window: None,
        })
    }

    pub fn run<G: CanvasHandler>(&mut self, handler: &mut G) -> Result<(), String> {
        let event_loop = EventLoop::new().map_err(|error| error.to_string())?;

        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(
                self.canvas.width() as u32,
                self.canvas.height() as u32,
            ))
            .build(&event_loop)
            .map_err(|error| error.to_string())?;

        let window = Arc::new(window);

        self.window = Some(window.clone());

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
                        handler.update(self.canvas);
                        let mut buffer = surface.buffer_mut().expect("could not get mut buffer!");
                        for index in 0..self.canvas.width() * self.canvas.height() {
                            buffer[index as usize] = self.canvas.color_at(index);
                        }

                        buffer.present().unwrap();
                    }
                    _ => (),
                },
                _ => window.request_redraw(),
            })
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}

impl<'a, T: RequestDraw<'a> + Canvas> HandlesDrawRequest for Graphics<'a, T> {
    fn draw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw()
        }
    }
}

unsafe impl<'a, T: RequestDraw<'a> + Canvas> Sync for Graphics<'a, T> {}
