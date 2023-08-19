use std::time::Duration;

use crossterm::event;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

pub trait Window {
    fn new(x: u16, y: u16, height: u16, width: u16) -> Self;
    fn height(&self) -> &u16;
    fn width(&self) -> &u16;
    fn get_window(&self) -> Rect;
}

pub struct GameWindow {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    title: String,
    layout: Layout,
    colour: Color,
}

impl Window for GameWindow {
    fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
        Self {
            x,
            y,
            height,
            width,
            title: String::from("TYPING TEST"),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(width)]),
            colour: Color::Cyan,
        }
    }

    fn get_window(&self) -> Rect {
        self.layout.split(Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })[0]
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }
}

pub struct InputWindow {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    title: String,
    layout: Layout,
    colour: Color,
}

impl Window for InputWindow {
    fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
        Self {
            x,
            y,
            height,
            width,
            title: String::from("INPUT"),
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(100)]),
            colour: Color::Green,
        }
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }

    fn get_window(&self) -> Rect {
        self.layout.split(Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })[0]
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
    layout: Layout,
    colour: Color,
    x: u16,
    y: u16,
    height: u16,
    width: u16,
}

impl Window for InfoWindow {
    fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
        Self {
            x,
            y,
            height,
            width,
            title: String::from("INFO"),
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(100)]),
            colour: Color::Red,
        }
    }

    fn height(&self) -> &u16 {
        &self.height
    }

    fn width(&self) -> &u16 {
        &self.width
    }

    fn get_window(&self) -> Rect {
        self.layout.split(Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })[0]
    }
}

struct Timer {
    minutes: u8,
    seconds: u8,
}
