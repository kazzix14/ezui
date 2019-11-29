use ezui::mouse::*;
use ezui::standard::*;
use ezui::widget::*;
use ezui::*;

use glium;
use glium::glutin;
use glium::DisplayBuild;
use glium::Surface;

pub const FONT_RAW: &'static [u8] = include_bytes!("resource/OpenSans-Regular.ttf");
pub const KNOB_BASE_WHITE_RAW: &'static [u8] = include_bytes!("resource/white.png");
pub const KNOB_BASE_BLACK_RAW: &'static [u8] = include_bytes!("resource/black.png");
pub const KNOB_LIGHT_RAW: &'static [u8] = include_bytes!("resource/light.png");

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(640, 640)
        .with_vsync()
        .build_glium()
        .unwrap();

    let mut ui = Ui::new(&display);

    let font =
        FontTexture::new(&display, FONT_RAW, 50, FontTexture::ascii_character_list()).unwrap();

    let text_display = ui.build_text_display(&font, "hello");

    let mut text = UiTextBuilder::default()
        .position((0.1, 0.1))
        .size((0.15, 0.1))
        .text(text_display)
        .color((0.1, 0.1, 0.1, 1.0))
        .build()
        .unwrap();

    let mut button1 = UiButtonBuilder::default()
        .position((0.1, 0.1))
        .size((0.15, 0.1))
        .build()
        .unwrap();

    let mut button2 = UiButtonBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .build()
        .unwrap();

    let texture = SimpleTexture::from(&KNOB_BASE_WHITE_RAW, ImageFormat::PNG, &display).unwrap();
    let knob_base = UiTextureBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .rotation(0.0)
        .texture(&texture)
        .build()
        .unwrap();

    let texture = SimpleTexture::from(&KNOB_LIGHT_RAW, ImageFormat::PNG, &display).unwrap();
    let mut knob_light = UiTextureBuilder::default()
        .position((0.5, 0.5))
        .size((0.3, 0.3))
        .rotation(0.0)
        .texture(&texture)
        .build()
        .unwrap();

    loop {
        ui.update(|target, mouse, system| {
            button1.update(&mouse);
            button2.update(&mouse);

            target.clear_color(0.4, 0.4, 0.4, 1.0);

            match *button1.left() {
                ButtonState::Pressed(_x, _y) => {
                    text.set_text("world");
                    text.color.0 += mouse.delta_position().0 * 2.55;
                    text.color.1 += mouse.delta_position().1 * 2.55;
                }
                _ => {
                    text.set_text("hello");
                }
            }
            match *button1.right() {
                ButtonState::Pressed(_x, _y) => {
                    text.set_text("MOVE!");
                    text.position.0 += mouse.delta_position().0 * 5.0;
                    text.position.1 += mouse.delta_position().1 * 5.0;
                    button1.position.0 += mouse.delta_position().0 * 5.0;
                    button1.position.1 += mouse.delta_position().1 * 5.0;
                }
                _ => {
                    text.set_text("hello");
                }
            }

            match *button2.left() {
                ButtonState::Pressed(_x, _y) => {
                    knob_light.rotation += mouse.delta_position().1 * 360.0;
                }
                _ => (),
            }
            match *button2.right() {
                ButtonState::Pressed(_x, _y) => {
                    knob_light.rotation += mouse.delta_position().1 * 360.0 * 2.0;
                }
                _ => (),
            }

            text.draw(target, system);
            knob_base.draw(target, system);
            knob_light.draw(target, system);
        });
    }
}
