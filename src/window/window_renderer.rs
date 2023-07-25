use std::io;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction};
use tui::widgets::{Block, Borders};
use tui::Frame;
use tui::{layout::Layout, Terminal};

use super::window::{GameWindow, InfoWindow, InputWindow, Window};

type CrossTermTerminal = Terminal<CrosstermBackend<io::Stdout>>;
pub struct WindowRenderer {
    game_win: GameWindow,
    input_win: InputWindow,
    info_win: InfoWindow,
}

impl WindowRenderer {
    pub fn new(terminal: &mut CrossTermTerminal) -> Result<Self, io::Error> {
        let game_win = GameWindow::new(0, 0);
        let input_win = InputWindow::new(0, 0);
        let info_win = InfoWindow::new(0, 0);
        let _ = terminal.draw(|frame| {
            let windows = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let block = Block::default().title("SOMETHING").borders(Borders::ALL);

            for window in windows {
                frame.render_widget(block.clone(), window.clone());
            }
        });

        Ok(Self {
            game_win,
            input_win,
            info_win,
        })
    }
}
