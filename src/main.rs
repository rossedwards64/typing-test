#![allow(dead_code, unused)]

mod game;
mod window;

use std::{io, thread, time::Duration};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Terminal,
};

use game::{game_logic, word_file::WordFile};
use window::window_renderer;

static PATH: &str = "data/words.txt";

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_tui()?;
    let mut renderer = window_renderer::WindowRenderer::new(&mut terminal);

    let mut word_file = WordFile::new(PATH);
    let word = word_file.get_random_word();

    renderer.render_windows();

    game_logic::game_loop();

    thread::sleep(Duration::from_millis(5000));


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
