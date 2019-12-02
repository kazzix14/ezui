use crate::drawable::*;
use crate::mouse::*;
use crate::widget::*;

use std::sync::Arc;

use derive_builder::Builder;
use glium_text_rusttype::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UiTexture {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub rotation: f32,
    texture: Arc<SimpleTexture>,
}

impl Widget for UiTexture {
    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }

    fn drawable(&self) -> Option<Drawable> {
        Some(Drawable::Texture(Arc::clone(&self.texture)))
    }
    fn matrix(&self) -> [[f32; 4]; 4] {
        let mat = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let mat = self.scale(mat);
        let mat = self.translate(mat);
        let mat = self.rotate(mat);
        mat
    }
}

impl Rotatable for UiTexture {
    fn rotation(&self) -> f32 {
        self.rotation
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UiButton {
    pub position: (f32, f32),
    pub size: (f32, f32),
    #[builder(setter(skip))]
    left: ButtonState,
    #[builder(setter(skip))]
    middle: ButtonState,
    #[builder(setter(skip))]
    right: ButtonState,
}

impl Widget for UiButton {
    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }

    fn drawable(&self) -> Option<Drawable> {
        None
    }
}

impl Pressable for UiButton {
    fn state(&self) -> [(MouseButton, ButtonState); 3] {
        [
            (MouseButton::Left, self.left),
            (MouseButton::Middle, self.middle),
            (MouseButton::Right, self.right),
        ]
    }

    fn update(&mut self, mouse: &MouseStatus) {
        macro_rules! update {
            ($name:ident) => {
                self.$name = ButtonState::Released;
                if let ButtonState::Pressed(x, y) = mouse.button().$name() {
                    if collision_check_rect_point(self.position(), self.size(), (x, y)) {
                        let pos = mouse.position();
                        self.$name = ButtonState::Pressed(pos.0 - x, pos.1 - y);
                    }
                }
            };
        }

        update!(left);
        update!(middle);
        update!(right);
    }
}

#[derive(Builder)]
#[builder(build_fn(skip), pattern = "owned")]
pub struct UiText {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: (f32, f32, f32, f32),

    text: Box<TextDisplay<Arc<FontTexture>>>,
}

#[allow(dead_code)]
impl UiTextBuilder {
    pub fn build(self) -> Result<UiText, String> {
        let text = self.text.ok_or("text must be inititalized")?;
        Ok(UiText {
            position: self.position.ok_or("position must be inititalized")?,
            size: self.size.ok_or("size must be inititalized")?,
            color: self.color.ok_or("color must be inititalized")?,
            text: text,
        })
    }
}

impl UiText {
    pub fn set_text(&mut self, string: &'static str) {
        self.text.set_text(string);
    }
}

impl Widget for UiText {
    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }

    fn size_adjusted(&self) -> (f32, f32) {
        let (w, h) = self.size();
        let tw = self.text.get_width();
        (2.0 * w / tw, 2.0 * h)
    }

    fn position_adjusted(&self) -> (f32, f32) {
        let (x, y) = self.position();
        let (x, y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        let (_w, h) = self.size_adjusted();

        let (x, y) = (x, y + h);
        (x, y)
    }

    fn drawable(&self) -> Option<Drawable> {
        Some(Drawable::from_font(&self.text, self.color))
    }
}

fn collision_check_rect_point(
    rect_pos: (f32, f32),
    rect_size: (f32, f32),
    point: (f32, f32),
) -> bool {
    let (x, y) = point;
    let (left, top) = rect_pos;
    let (right, bottom) = (left + rect_size.0, top + rect_size.1);

    if left <= x && x <= right && top <= y && y <= bottom {
        true
    } else {
        false
    }
}
