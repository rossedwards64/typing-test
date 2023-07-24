pub(crate) trait Window {
    fn new(height: u16, width: u16) -> Self;
}

pub struct GameWindow {
    height: u16,
    width: u16,
}

impl Window for GameWindow {
    fn new(height: u16, width: u16) -> GameWindow {
        GameWindow { height, width }
    }
}

pub struct InputWindow {
    height: u16,
    width: u16,
}

impl Window for InputWindow {
    fn new(height: u16, width: u16) -> InputWindow {
        InputWindow { height, width }
    }
}

pub struct InfoWindow {
    height: u16,
    width: u16,
}

impl Window for InfoWindow {
    fn new(height: u16, width: u16) -> InfoWindow {
        InfoWindow { height, width }
    }
}

enum Colours {}

struct Timer {
    minutes: u8,
    seconds: u8,
}
