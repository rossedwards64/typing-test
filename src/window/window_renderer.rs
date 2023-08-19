use std::io;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{layout::Layout, Terminal};

use super::window::{GameWindow, InfoWindow, InputWindow, Window};

type CrossTermTerminal = Terminal<CrosstermBackend<io::Stdout>>;
pub struct WindowRenderer<'a> {
    terminal: &'a mut CrossTermTerminal,
    game_win: GameWindow,
    input_win: InputWindow,
    info_win: InfoWindow,
    width: u16,
    height: u16,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(terminal: &'a mut CrossTermTerminal) -> Result<Self, io::Error> {
        let (width, height) = match terminal.size() {
            Ok(rect) => (rect.width, rect.height),
            Err(err) => panic!("Couldn't get terminal size! {err}"),
        };

        let game_win = GameWindow::new(0, 0, height.saturating_sub(6), width);

        let input_win = InputWindow::new(0, 22, 5, width.saturating_div(2));

        let info_win = InfoWindow::new(width.saturating_div(2), 22, 8, width.saturating_div(2));

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
        self.render_game_win();
        self.render_input_win();
        self.render_info_win();

        let result = self.terminal.draw(|frame| {
            frame.render_widget(
                Block::default().borders(Borders::all()).title("GAME"),
                self.game_win.get_window(),
            );
            frame.render_widget(
                Block::default().borders(Borders::all()).title("INPUT"),
                self.input_win.get_window(),
            );
            frame.render_widget(
                Block::default().borders(Borders::all()).title("INFO"),
                self.info_win.get_window(),
            );
        });

        if result.is_err() {}
    }

    fn render_game_win(&self) {}

    fn render_input_win(&self) {}

    fn render_info_win(&self) {}
}
