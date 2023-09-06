use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use std::io;
use thiserror::Error;

use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph};
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

    pub fn render_windows(&mut self, input: &str) {
        let make_block = |title, colour| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD).fg(colour),
                ))
        };

        let words = Paragraph::new("")
            .block(make_block("GAME", Color::Red))
            .scroll((0, 0));

        let input = Paragraph::new(input)
            .block(make_block("INPUT", Color::Green))
            .style(Style::default());

        let info = Paragraph::new(Line::from(vec![
            format!("Score: 0").into(),
            format!("Time: 0:00").into(),
            format!("Words: 0").into(),
            format!("WPM: 0").into(),
            format!("X/Y: 0, 0").into(),
        ]))
        .block(make_block("INFO", Color::Cyan));

        let _ = self.terminal.draw(|frame| {
            frame.render_widget(words, self.game_win.get_window());
            frame.render_widget(input, self.input_win.get_window());
            frame.render_widget(info, self.info_win.get_window());
        });
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error("Error rendering window!")]
    RenderError,
    #[error("Unkown error occurred.")]
    Unknown,
}
