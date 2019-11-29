use glium;
use glium_text_rusttype as text;
use image;

use glium::backend::Facade;
use glium::texture::CompressedSrgbTexture2d;
use glium::texture::RawImage2d;
use glium::texture::TextureCreationError;
use glium::uniforms::AsUniformValue;
use glium::uniforms::UniformValue;
use image::ImageError;
use std::io::Cursor;
use text::FontTexture;
use text::TextDisplay;

pub use image::ImageFormat;

#[derive(Clone)]
pub enum Drawable<'a> {
    Texture(&'a SimpleTexture),
    Text(&'a TextDisplay<&'a FontTexture>, (f32, f32, f32, f32)),
}

impl<'a> Drawable<'a> {
    pub fn from_texture(texture: &'a SimpleTexture) -> Self {
        Drawable::Texture(texture)
    }

    pub fn from_font(text: &'a TextDisplay<&FontTexture>, color: (f32, f32, f32, f32)) -> Self {
        Drawable::Text(text, color)
    }
}

pub struct SimpleTexture(CompressedSrgbTexture2d);

#[derive(Debug)]
pub enum SimpleTextureError {
    ImageError(ImageError),
    TextureCreationError(TextureCreationError),
}

impl SimpleTexture {
    pub fn from<T, F>(
        texture_raw: &T,
        format: ImageFormat,
        display: &F,
    ) -> Result<Self, SimpleTextureError>
    where
        T: AsRef<[u8]>,
        F: Facade,
    {
        use SimpleTextureError::*;

        let image = match image::load(Cursor::new(texture_raw), format) {
            Ok(img) => img.to_rgba(),
            Err(e) => return Err(ImageError(e)),
        };

        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        match CompressedSrgbTexture2d::new(display, image) {
            Ok(texture) => Ok(SimpleTexture(texture)),
            Err(e) => Err(TextureCreationError(e)),
        }
    }
}

impl AsUniformValue for SimpleTexture {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::CompressedSrgbTexture2d(&self.0, None)
    }
}

impl AsUniformValue for &SimpleTexture {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::CompressedSrgbTexture2d(&self.0, None)
    }
}
