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
    widgets::{Block, Borders},
    Terminal,
};

use game::{game_logic, word_file::WordFile};

static PATH: &str = "data/words.txt";

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    let mut word_file = WordFile::new(PATH);
    let word = word_file.get_random_word();
    println!("{word}");
    game_logic::game_loop();
    Ok(())
}
