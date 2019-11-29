use crate::drawable::*;
use crate::mouse::*;
use crate::system::*;

pub trait Widget {
    fn draw(&self, target: &mut glium::Frame, system: &System) {
        if let Some(mut drawable) = self.drawable() {
            let matrix = self.matrix();
            system.draw::<glium::Frame, [[f32; 4]; 4]>(&mut drawable, target, matrix);
        }
    }
    fn position(&self) -> (f32, f32);
    fn size(&self) -> (f32, f32);
    fn size_adjusted(&self) -> (f32, f32) {
        let (w, h) = self.size();
        (w * 2.0, h * 2.0)
    }
    fn position_adjusted(&self) -> (f32, f32) {
        let (x, y) = self.position();
        let (x, y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);
        let (w, h) = self.size_adjusted();
        let (x, y) = (x + w / 2.0, y + h / 2.0);
        (x, y)
    }
    fn translate(&self, mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let (x, y) = self.position_adjusted();

        let translation = nalgebra::Vector3::new(x, -y, 0.0);

        let mat = nalgebra::Matrix4::from(mat);
        let mat = mat.append_translation(&translation);
        mat.into()
    }
    fn scale(&self, mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut mat = mat;
        let (w, h) = self.size_adjusted();

        mat[0][0] *= w;
        mat[1][1] *= h;

        mat
    }
    fn drawable(&self) -> Option<Drawable>;
    fn matrix(&self) -> [[f32; 4]; 4] {
        let mat = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let mat = self.translate(mat);
        let mat = self.scale(mat);
        mat
    }
}

pub trait Pressable: Widget {
    fn left(&self) -> &ButtonState;
    fn middle(&self) -> &ButtonState;
    fn right(&self) -> &ButtonState;
    fn update(&mut self, mouse: &MouseStatus);
}

pub trait Rotatable: Widget {
    fn rotation(&self) -> f32;
    fn rotate(&self, mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let rotation = self.rotation() / 360.0 * 2.0 * std::f32::consts::PI;
        let rotation = nalgebra::Matrix4::from_euler_angles(0.0, 0.0, rotation);

        let mat = nalgebra::Matrix4::from(mat);
        let mat = mat * rotation;
        mat.into()
    }
}
