use std::time::Duration;
use crossterm::event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

pub struct Window {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    title: String,
    layout: Layout,
    colour: Color,
}

impl Window {
    pub fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
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

    pub fn get_window(&self) -> Rect {
        self.layout.split(Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        })[0]
    }

    pub fn height(&self) -> &u16 {
        &self.height
    }

    pub fn width(&self) -> &u16 {
        &self.width
    }
}
