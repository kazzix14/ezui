use crate::drawable::*;
use crate::widget::*;

use std::sync::Arc;

use derive_builder::Builder;

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
