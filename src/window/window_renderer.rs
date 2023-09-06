use std::io;
use thiserror::Error;

use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;

use super::window::Window;

type CrossTermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct WindowRenderer<'a> {
    terminal: &'a mut CrossTermTerminal,
    game_win: Window,
    input_win: Window,
    info_win: Window,
    width: u16,
    height: u16,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(terminal: &'a mut CrossTermTerminal) -> Result<Self, io::Error> {
        let (width, height) = match terminal.size() {
            Ok(rect) => (rect.width, rect.height),
            Err(err) => panic!("Couldn't get terminal size! {err}"),
        };

        let game_win = Window::new(0, 0, height.saturating_sub(6), width);

        let input_win = Window::new(0, 22, 5, width.saturating_div(2));

        let info_win = Window::new(width.saturating_div(2), 22, 8, width.saturating_div(2));

        Ok(Self {
            terminal,
            game_win,
            input_win,
            info_win,
            width,
            height,
        })
    }

    pub fn render_windows(&mut self) {
        let result = self.terminal.draw(|frame| {
            // RENDER GAME WINDOW
            frame.render_widget(
                Block::default().borders(Borders::all()).title("GAME"),
                self.game_win.get_window(),
            );

            // RENDER INPUT WINDOW
            frame.render_widget(
                Block::default().borders(Borders::all()).title("INPUT"),
                self.input_win.get_window(),
            );

            // RENDER INFO WINDOW
            frame.render_widget(
                Block::default().borders(Borders::all()).title("INFO"),
                self.info_win.get_window(),
            );
        });

        if result.is_err() {}
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error("Error rendering window!")]
    RenderError,
    #[error("Unkown error occurred.")]
    Unknown,
}
