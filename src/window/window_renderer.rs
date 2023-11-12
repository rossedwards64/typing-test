use super::window::Window;
use crate::game::game_logic::GameInfo;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use thiserror::Error;

type CrossTermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct WindowRenderer<'a> {
    terminal: &'a mut CrossTermTerminal,
    game_win: Window,
    input_win: Window,
    info_win: Window,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(terminal: &'a mut CrossTermTerminal) -> Result<Self, WindowRendererError> {
        let (width, height) = match terminal.size() {
            Ok(rect) => (rect.width, rect.height),
            Err(err) => return Err(WindowRendererError::InvalidTerminalSize(err)),
        };

        let lower_win_pos: u16 = 22;
        let game_win = Window::new(0, 0, height.saturating_sub(6), width);

        let input_win = Window::new(0, lower_win_pos, 5, width.saturating_div(2));
        let info_win = Window::new(
            width.saturating_div(2),
            lower_win_pos,
            8,
            width.saturating_div(2),
        );

        Ok(Self {
            terminal,
            game_win,
            input_win,
            info_win,
        })
    }

    pub fn render_windows(
        &mut self,
        input: &str,
        game_info: &GameInfo,
    ) -> Result<(), WindowRendererError> {
        let make_block = |title, colour| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(Span::styled(
                    title,
                    Style::default()
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                        .fg(colour),
                ))
        };

        let words = Paragraph::new("")
            .block(make_block("TYPING TEST", Color::Red))
            .scroll((0, 0));

        let input = Paragraph::new(input)
            .block(make_block("INPUT", Color::Green))
            .style(Style::default());

        let info = Paragraph::new(Line::from(vec![
            format!("Score: {} ", game_info.score).into(),
            format!("Time: {} ", game_info.timer.to_string()).into(),
            format!("Words: {} ", game_info.words).into(),
            format!("WPM: {}", game_info.wpm).into(),
        ]))
        .block(make_block("INFO", Color::Cyan));

        match self.terminal.draw(|frame| {
            frame.render_widget(words, self.game_win.get_window());
            frame.render_widget(input, self.input_win.get_window());
            frame.render_widget(info, self.info_win.get_window());
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(WindowRendererError::FailedToRender(e)),
        }
    }
}

#[derive(Error, Debug)]
pub enum WindowRendererError {
    #[error("Couldn't get terminal size. {0}")]
    InvalidTerminalSize(io::Error),
    #[error("Error rendering window. {0}")]
    FailedToRender(io::Error),
}
