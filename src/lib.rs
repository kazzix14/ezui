mod drawable;
pub mod mouse;
mod resource;
pub mod standard;
mod system;
pub mod widget;

pub use drawable::*;
pub use glium;
use glium::backend::glutin as glutin_backend;
use glium::backend::glutin::glutin;
use glium_text_rusttype;
pub use glium_text_rusttype::*;
use image;
pub use image::ImageFormat;
use mouse::*;
use std::sync::Arc;
use system::System;
pub use winit;

pub struct Ui {
    mouse: MouseStatus,
    display: glutin_backend::Display,
    events_loop: glutin::EventsLoop,
    system: System,
}

impl Ui {
    pub fn new(display: glutin_backend::Display, events_loop: glutin::EventsLoop) -> Ui {
        let system = System::new(&display);

        Ui {
            mouse: MouseStatus::default(),
            display,
            events_loop,
            system,
        }
    }

    pub fn update<F>(&mut self, update_func: F)
    where
        F: FnOnce(
            &mut glium::Frame,
            &mut std::vec::IntoIter<glutin::Event>,
            &mut MouseStatus,
            &mut System,
        ),
    {
        let display = &self.display;
        let events_loop = &mut self.events_loop;
        let mouse = &mut self.mouse;
        let system = &mut self.system;

        let mut events = {
            let mut events1 = Vec::new();
            let mut events2 = Vec::new();
            events_loop.poll_events(|event| {
                events1.push(event.clone());
                events2.push(event);
            });
            (events1.into_iter(), events2.into_iter())
        };

        let mut target = display.draw();
        mouse.update(&target, &mut events.0);

        update_func(&mut target, &mut events.1, mouse, system);
        target.finish().unwrap();
    }

    pub fn build_text_display(
        &self,
        font: Arc<glium_text_rusttype::FontTexture>,
        text: &'static str,
    ) -> TextDisplay<Arc<FontTexture>> {
        TextDisplay::new(&self.system.text_system, font, text)
    }
}
