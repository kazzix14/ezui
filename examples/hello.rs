use ezui::mouse::*;
use ezui::standard::*;
use ezui::widget::*;
use ezui::*;

use ezui::glium::glutin;
//use ezui::glium::DisplayBuild;
use ezui::glium::Surface;

use winit::dpi::LogicalSize;

use std::sync::Arc;

pub const FONT_RAW: &'static [u8] = include_bytes!("resource/OpenSans-Regular.ttf");
pub const KNOB_BASE_WHITE_RAW: &'static [u8] = include_bytes!("resource/white.png");
pub const KNOB_BASE_BLACK_RAW: &'static [u8] = include_bytes!("resource/black.png");
pub const KNOB_LIGHT_RAW: &'static [u8] = include_bytes!("resource/light.png");

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("hello")
        .with_dimensions(LogicalSize::new(640.0, 640.0));

    let context_builder = glutin::ContextBuilder::new();

    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let texture_knob_base =
        SimpleTexture::from(&KNOB_BASE_WHITE_RAW, ImageFormat::PNG, &display).unwrap();

    let texture_knob_light =
        SimpleTexture::from(&KNOB_LIGHT_RAW, ImageFormat::PNG, &display).unwrap();

    let font =
        FontTexture::new(&display, FONT_RAW, 50, FontTexture::ascii_character_list()).unwrap();

    let mut ui = Ui::new(display, events_loop);

    let text_display = ui.build_text_display(Arc::new(font), "hello");

    let mut text = UiTextBuilder::default()
        .position((0.1, 0.1))
        .size((0.15, 0.1))
        .text(Box::new(text_display))
        .color((0.1, 0.1, 0.1, 1.0))
        .build()
        .unwrap();

    let mut button = UiButtonBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .build()
        .unwrap();

    let knob_base = UiTextureBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .rotation(0.0)
        .texture(Arc::new(texture_knob_base))
        .build()
        .unwrap();

    let mut knob_light = UiTextureBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .rotation(0.0)
        .texture(Arc::new(texture_knob_light))
        .build()
        .unwrap();

    let mut exit = false;
    while !exit {
        ui.update(|target, events, mouse, system| {
            events.for_each(|ev| {
                if let glutin::Event::WindowEvent { event, .. } = ev {
                    match event {
                        glutin::WindowEvent::CloseRequested => exit = true,
                        _ => (),
                    }
                }
            });

            button.update(&mouse);

            target.clear_color(0.4, 0.4, 0.4, 1.0);

            button
                .state()
                .iter()
                .for_each(|(button, state)| match state {
                    ButtonState::Pressed(_x, _y) => {
                        let scale = match button {
                            MouseButton::Left => 1.0,
                            MouseButton::Middle => 0.25,
                            MouseButton::Right => 3.0,
                        };

                        knob_light.rotation += mouse.delta_position().1 * 360.0 * scale;
                    }
                    _ => (),
                });

            text.color.1 = mouse.position().0;
            text.color.2 = mouse.position().1;

            text.draw(target, system);
            knob_base.draw(target, system);
            knob_light.draw(target, system);
        });
    }
}
