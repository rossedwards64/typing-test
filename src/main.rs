mod game;
mod window;

use std::{
    io::{self},
    thread,
    time::Duration,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use game::{game_logic, word_file::WordFile};
use window::window_renderer::{self};

static PATH: &str = "data/words.txt";

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_tui()?;

    let renderer = window_renderer::WindowRenderer::new(&mut terminal);
    let file = WordFile::new(PATH);

    game_logic::game_loop(file, renderer?);

    thread::sleep(Duration::from_secs(5));

    destroy_tui(terminal)?;
    Ok(())
}

fn setup_tui() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn destroy_tui(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;
    Ok(())
}
