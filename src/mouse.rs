use glium::backend::glutin::glutin;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct MouseStatus {
    position: (f32, f32),
    delta_position: (f32, f32),
    button: MouseButtonStatus,
}

impl MouseStatus {
    pub fn update(
        &mut self,
        target: &glium::Frame,
        events: &mut std::vec::IntoIter<glutin::Event>,
    ) {
        use glium::Surface;
        use glutin::Event::*;
        use glutin::WindowEvent::*;
        use winit::dpi::LogicalPosition;

        self.delta_position = (0.0, 0.0);

        events.for_each(|ev| {
            if let WindowEvent { event: ev, .. } = ev {
                match ev {
                    CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => {
                        let (w, h) = target.get_dimensions();
                        let (w, h) = (w as f32, h as f32);
                        let (x, y) = (x as f32, y as f32);
                        let (x, y) = (x / w, y / h);
                        self.delta_position = (x - self.position.0, y - self.position.1);
                        self.position = (x, y);
                    }
                    MouseInput { state, button, .. } => {
                        self.button.update(state, button, self.position)
                    }
                    _ => (),
                }
            }
        });
    }
    pub fn position(&self) -> (f32, f32) {
        self.position
    }
    pub fn delta_position(&self) -> (f32, f32) {
        self.delta_position
    }
    pub fn button(&self) -> MouseButtonStatus {
        self.button
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct MouseButtonStatus {
    left: ButtonState,
    right: ButtonState,
    middle: ButtonState,
}

impl MouseButtonStatus {
    fn update(
        &mut self,
        state: glutin::ElementState,
        button: glutin::MouseButton,
        position: (f32, f32),
    ) {
        use glutin::ElementState;
        use glutin::MouseButton;

        let state = match state {
            ElementState::Pressed => ButtonState::Pressed(position.0, position.1),
            ElementState::Released => ButtonState::Released,
        };

        match button {
            MouseButton::Left => self.left = state,
            MouseButton::Right => self.right = state,
            MouseButton::Middle => self.middle = state,
            _ => (),
        }
    }

    pub fn left(&self) -> ButtonState {
        self.left
    }
    pub fn middle(&self) -> ButtonState {
        self.middle
    }
    pub fn right(&self) -> ButtonState {
        self.right
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonState {
    Pressed(f32, f32),
    Released,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Released
    }
}
