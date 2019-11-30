use crate::system::image;
use crate::Drawable;

use glium_text_rusttype as text;

use glium::backend::Facade;
use glium::uniform;
use glium::uniforms::AsUniformValue;
use glium::DrawParameters;
use glium::Surface;

pub struct System {
    pub image_system: image::ImageSystem,
    pub text_system: text::TextSystem,
}

impl System {
    pub fn new<F>(display: &F) -> Self
    where
        F: Facade,
    {
        let image_system = image::ImageSystem::new(display);
        let text_system = text::TextSystem::new(display);

        System {
            image_system: image_system,
            text_system: text_system,
        }
    }

    pub fn draw<S: ?Sized, M>(&self, drawable: &mut Drawable, target: &mut S, matrix: M)
    where
        S: Surface,
        M: Into<[[f32; 4]; 4]> + AsUniformValue,
    {
        match drawable {
            Drawable::Text(text, color) => {
                let sys = &self.text_system;
                text::draw(text, sys, target, matrix, *color);
            }
            Drawable::Texture(texture) => {
                let sys = &self.image_system;

                let vb = &sys.vertex_buffer;
                let ib = &sys.index_buffer;
                let prog = &sys.program;

                let uniforms = &uniform!(matrix: matrix, tex: texture.as_ref());

                let params = &DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                };

                target.draw(vb, ib, prog, uniforms, params).unwrap();
            }
        }
    }
}
