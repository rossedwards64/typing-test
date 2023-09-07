use std::{
    path::Path,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use crossterm::event;
use thiserror::Error;

use super::{word_file::WordFile, word_manager::WordManager};
use crate::window::window_renderer::{WindowRenderer, WindowRendererError};

struct GameState<'a, P>
where
    P: AsRef<Path>,
{
    word_manager: WordManager<P>,
    renderer: WindowRenderer<'a>,
    input_buf: String,
    terminate: bool,
}

pub struct GameInfo {
    pub score: u32,
    pub timer: Timer,
    pub words: u32,
    pub wpm: u32,
}

#[derive(Clone, Copy)]
pub struct Timer {
    minutes: u32,
    seconds: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            minutes: 0,
            seconds: 0,
        }
    }

    pub fn increment_timer(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.minutes += 1;
            self.seconds = 0;
        }
    }
}

impl ToString for Timer {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        if self.minutes < 10 && self.seconds < 10 {
            buf = format!("0{}:0{}", self.minutes, self.seconds);
        } else if self.minutes >= 10 && self.seconds < 10 {
            buf = format!("{}:0{}", self.minutes, self.seconds);
        } else if self.minutes < 10 && self.seconds >= 10 {
            buf = format!("0{}:{}", self.minutes, self.seconds);
        } else if self.minutes >= 10 && self.seconds >= 10 {
            buf = format!("{}:{}", self.minutes, self.seconds);
        }
        buf
    }
}

pub fn game_loop<P>(file: WordFile<P>, renderer: WindowRenderer<'_>) -> Result<(), GameLoopError>
where
    P: AsRef<Path>,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut timer = Timer {
            minutes: 0,
            seconds: 0,
        };

        loop {
            timer.increment_timer();
            let res = tx.send(timer);
            thread::sleep(Duration::from_secs(1));

            if res.is_err() {
                continue;
            }
        }
    });

    let mut game_state = GameState {
        word_manager: WordManager::new(file),
        renderer,
        input_buf: String::new(),
        terminate: false,
    };

    let mut game_info = GameInfo {
        score: 0,
        timer: Timer::new(),
        words: 0,
        wpm: 0,
    };

    loop {
        update(&mut game_state, &mut game_info, &rx)?;
        input(&mut game_state)?;
        render(&mut game_state, &game_info)?;

        if game_state.terminate {
            break;
        }
    }

    Ok(())
}

fn update<P>(
    game_state: &mut GameState<P>,
    game_info: &mut GameInfo,
    rx: &Receiver<Timer>,
) -> Result<(), GameLoopError>
where
    P: AsRef<Path>,
{
    game_state.word_manager.spawn_word();

    let _ = rx.try_recv().map(|timer| {
        game_info.timer = timer;
    });
    Ok(())
}

fn input<P>(game_state: &mut GameState<P>) -> Result<(), GameLoopError>
where
    P: AsRef<Path>,
{
    if matches!(event::poll(Duration::from_millis(100)), Ok(true)) {
        if let Ok(event::Event::Key(key)) = event::read() {
            match key.code {
                event::KeyCode::Char(c) => game_state.input_buf.push(c),
                event::KeyCode::Backspace => {
                    game_state.input_buf.pop();
                }
                event::KeyCode::Esc => {
                    game_state.terminate = true;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

fn render<P>(game_state: &mut GameState<P>, game_info: &GameInfo) -> Result<(), GameLoopError>
where
    P: AsRef<Path>,
{
    game_state
        .renderer
        .render_windows(&game_state.input_buf, game_info)
        .map_err(|err| GameLoopError::FailedToRender(err))
}

#[derive(Debug, Error)]
pub enum GameLoopError {
    #[error("Failed to update game state.")]
    FailedToUpdate(#[from] std::io::Error),
    #[error("Failed to render game state to terminal.")]
    FailedToRender(WindowRendererError),
}
