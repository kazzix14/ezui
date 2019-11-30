mod drawable;
pub mod mouse;
mod resource;
pub mod standard;
mod system;
pub mod widget;

pub use drawable::*;
pub use glium;
use glium::backend::glutin_backend;
pub use glium_text::*;
use glium_text_rusttype as glium_text;
use image;
pub use image::ImageFormat;
use mouse::*;
use system::System;
pub use winit;

pub struct Ui<'a> {
    mouse: MouseStatus,
    display: &'a glutin_backend::GlutinFacade,
    system: System,
}

impl<'a> Ui<'a> {
    pub fn new(display: &'a glutin_backend::GlutinFacade) -> Ui {
        let system = System::new(display);

        Ui {
            mouse: MouseStatus::default(),
            display: display,
            system: system,
        }
    }

    pub fn update<F>(&mut self, update_func: F)
    where
        F: FnOnce(
            &mut glium::Frame,
            &mut std::vec::IntoIter<winit::Event>,
            &mut MouseStatus,
            &mut System,
        ),
    {
        let display = self.display;
        let mouse = &mut self.mouse;
        let system = &mut self.system;

        let mut events = {
            let mut v1 = Vec::new();
            let mut v2 = Vec::new();
            for ev in display.poll_events() {
                v1.push(ev.clone());
                v2.push(ev);
            }
            (v1.into_iter(), v2.into_iter())
        };

        let mut target = display.draw();
        mouse.update(&target, &mut events.0);

        update_func(&mut target, &mut events.1, mouse, system);
        target.finish().unwrap();
    }

    pub fn build_text_display<'b>(
        &self,
        font: &'b glium_text_rusttype::FontTexture,
        text: &'b str,
    ) -> TextDisplay<&'b FontTexture> {
        TextDisplay::new(&self.system.text_system, font, text)
    }
}
