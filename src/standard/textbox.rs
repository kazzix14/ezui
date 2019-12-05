use crate::drawable::*;
use crate::widget::*;

use std::sync::Arc;

use derive_builder::Builder;
use glium_text_rusttype::*;

#[derive(Builder)]
#[builder(build_fn(skip), pattern = "owned")]
pub struct UiTextbox {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: (f32, f32, f32, f32),
    pub fontsize: f32,

    text: Box<TextDisplay<Arc<FontTexture>>>,
}

#[allow(dead_code)]
impl UiTextboxBuilder {
    pub fn build(self) -> Result<UiTextbox, String> {
        let text = self.text.ok_or("text must be inititalized")?;
        Ok(UiTextbox {
            position: self.position.ok_or("position must be inititalized")?,
            size: self.size.ok_or("size must be inititalized")?,
            color: self.color.ok_or("color must be inititalized")?,
            fontsize: self.fontsize.ok_or("fontsize must be inititalized")?,
            text: text,
        })
    }
}

impl UiTextbox {
    pub fn set_text(&mut self, string: &'static str) {
        self.text.set_text(string);
    }
}

impl Widget for UiTextbox {
    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }

    fn size_adjusted(&self) -> (f32, f32) {
        let fontsize = self.fontsize;
        let tw = 1.0;
        (2.0 * fontsize / tw, 2.0 * fontsize / tw)
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
