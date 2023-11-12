mod game;
mod window;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::{
    game_logic::{self, GameLoopError},
    word_file::WordFile,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use thiserror::Error;
use window::window_renderer::{self, WindowRendererError};

type CrossTermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

static PATH: &str = "data/words.txt";

fn main() -> Result<(), TypingTestError> {
    let mut terminal = setup_tui()?;
    let renderer = window_renderer::WindowRenderer::new(&mut terminal)?;
    let file = WordFile::new(PATH);

    game_logic::game_loop(file, renderer)?;

    destroy_tui(terminal)?;
    Ok(())
}

fn setup_tui() -> Result<CrossTermTerminal, TypingTestError> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn destroy_tui(mut terminal: CrossTermTerminal) -> Result<(), TypingTestError> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug, Error)]
enum TypingTestError {
    #[error("Error occurred during terminal configuration.")]
    TerminalConfiguration(#[from] io::Error),
    #[error("Error occurred during window renderer operation.")]
    WindowRendererProblem(#[from] WindowRendererError),
    #[error("Error occurred during game loop.")]
    GameError(#[from] GameLoopError),
}
