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
}

impl<'a> WindowRenderer<'a> {
    pub fn new(terminal: &'a mut CrossTermTerminal) -> Self {
        Self {
            terminal,
            game_win: GameWindow::new(0, 0),
            input_win: InputWindow::new(0, 0),
            info_win: InfoWindow::new(0, 0)
        }
    }

    pub fn render_windows(&mut self) {
        self.render_game_win();
        self.render_info_win();
        self.render_input_win();
    }

    fn render_game_win(&mut self) {
        let _ = self.terminal.draw(|frame| {
            let window = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(80)].as_ref())
                .split(frame.size());

            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new("TEXT HERE").alignment(Alignment::Center);

            frame.render_widget(block, window[0]);
            frame.render_widget(paragraph, window[0]);
        });
    }

    fn render_input_win(&mut self) {

    }

    fn render_info_win(&mut self) {

    }

    pub fn terminal(&mut self) -> &mut CrossTermTerminal {
        &mut self.terminal
    }
}
