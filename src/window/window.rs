use tui::style::Color;

pub trait Window {
    fn new(height: u16, width: u16) -> Self;
    fn height(&self) -> &u16;
    fn width(&self) -> &u16;
}

pub struct GameWindow {
    title: String,
    height: u16,
    width: u16,
    colour: Color,
}

impl Window for GameWindow {
    fn new(height: u16, width: u16) -> Self {
        Self {
            title: String::from("TYPING TEST"),
            height,
            width,
            colour: Color::Cyan,
        }
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }
}

pub struct InputWindow {
    title: String,
    height: u16,
    width: u16,
    colour: Color,
}

impl Window for InputWindow {
    fn new(height: u16, width: u16) -> Self {
        Self {
            title: String::from("INPUT"),
            height,
            width,
            colour: Color::Green,
        }
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }
}

pub struct InfoWindow {
    title: String,
    height: u16,
    width: u16,
    colour: Color,
}

impl Window for InfoWindow {
    fn new(height: u16, width: u16) -> Self {
        Self {
            title: String::from("INFO"),
            height,
            width,
            colour: Color::Red,
        }
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }
}

enum Colours {}

struct Timer {
    minutes: u8,
    seconds: u8,
}
