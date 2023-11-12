use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Window {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    layout: Layout,
}

impl Window {
    pub fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
        Self {
            x,
            y,
            height,
            width,
            layout: Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(width)]),
        }
    }

    pub fn get_window(&self) -> Rect {
        *self
            .layout
            .split(Rect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
            })
            .first()
            .unwrap()
    }
}
