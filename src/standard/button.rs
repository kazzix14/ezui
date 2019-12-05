use crate::drawable::*;
use crate::mouse::*;
use crate::standard::util;
use crate::widget::*;

use derive_builder::Builder;

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
                    if util::collision_check_rect_point(self.position(), self.size(), (x, y)) {
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
