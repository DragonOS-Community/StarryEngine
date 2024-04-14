use super::vector2::Vector2;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Init,

    Mouse {
        point: Vector2,
        left_button: bool,
        middle_button: bool,
        right_button: bool,
    },

    KeyPressed {
        character: Option<char>,
        scancode: u8,
    },

    KeyReleased {
        character: Option<char>,
        scancode: u8,
    },

    Scroll {
        x: i32,
        y: i32,
    },

    Resize {
        width: u32,
        height: u32,
    },

    Unknown,
}
