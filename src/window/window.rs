use std::time::Duration;

use crossterm::event;
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

impl InputWindow {
    pub fn get_input(&self) -> String {
        let mut buf = String::new();
        if matches!(event::poll(Duration::from_millis(100)), Ok(true)) {
            while let Ok(event) = event::read() {
                if let event::Event::Key(key) = event {
                    match key.code {
                        event::KeyCode::Char(c) => buf.push(c),
                        event::KeyCode::Backspace => {
                            buf.pop();
                        }
                        _ => (),
                    }
                }
            }
        }

        buf
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

struct Timer {
    minutes: u8,
    seconds: u8,
}
