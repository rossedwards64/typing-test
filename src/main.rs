mod game;
mod window;

use std::{io, thread, time::Duration};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use game::{
    game_logic::{self, GameLoopError},
    word_file::WordFile,
};
use thiserror::Error;
use window::window_renderer::{self, WindowRendererError};

static PATH: &str = "data/words.txt";

fn main() -> Result<(), TypingTestError> {
    let mut terminal = setup_tui()?;
    let renderer = window_renderer::WindowRenderer::new(&mut terminal)?;
    let file = WordFile::new(PATH);

    game_logic::game_loop(file, renderer)?;

    thread::sleep(Duration::from_secs(5));

    destroy_tui(terminal)?;
    Ok(())
}

fn setup_tui() -> Result<Terminal<CrosstermBackend<io::Stdout>>, TypingTestError> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn destroy_tui(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), TypingTestError> {
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
    TerminalError(#[from] io::Error),
    #[error("Error occurring during window renderer operation.")]
    RendererError(#[from] WindowRendererError),
    #[error("Error occurring during game loop.")]
    GameError(#[from] GameLoopError),
}
