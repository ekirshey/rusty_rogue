use super::utils::math::Vec2;

// Move this stuff to an input module
#[derive(PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Unknown
}

#[derive(PartialEq)]
pub enum MouseEvent {
    Press(MouseButton),
    Release(MouseButton),
    Hold(MouseButton),
    WheelUp,
    WheelDown,
}

#[derive(PartialEq)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Key(char),
    Mouse {
        offset: Vec2<usize>,
        position: Vec2<usize>,
        event: MouseEvent,
    },
    Unknown
}
