use super::{word_file::WordFile, word_manager::WordManager};
use crate::window::window_renderer::{WindowRenderer, WindowRendererError};
use crossterm::event;
use std::{
    any::Any,
    path::Path,
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use thiserror::Error;

type ThreadError = Box<dyn Any + Send>;

static TERMINATE: AtomicBool = AtomicBool::new(false);

struct GameState<'a, P>
where
    P: AsRef<Path>,
{
    word_manager: WordManager<P>,
    renderer: WindowRenderer<'a>,
    input_buf: String,
    timer_thread: JoinHandle<()>,
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
    pub const fn new() -> Self {
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

impl Default for Timer {
    fn default() -> Self {
        Self::new()
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
    let timer_thread = thread::spawn(move || {
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

            if TERMINATE.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
        }
    });

    let mut game_state = GameState {
        word_manager: WordManager::new(file),
        renderer,
        input_buf: String::new(),
        timer_thread,
    };

    let mut game_info = GameInfo {
        score: 0,
        timer: Timer::default(),
        words: 0,
        wpm: 0,
    };

    while !TERMINATE.load(std::sync::atomic::Ordering::Acquire) {
        update(&mut game_state, &mut game_info, &rx)?;
        input(&mut game_state)?;
        render(&mut game_state, &game_info)?;
    }

    end_game(game_state).map_or_else(|| Ok(()), |err| Err(err))
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
                event::KeyCode::Enter => game_state.input_buf.clear(),
                event::KeyCode::Esc => {
                    TERMINATE.store(true, std::sync::atomic::Ordering::Release);
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
        .map_err(GameLoopError::RenderingError)
}

fn end_game<P>(game_state: GameState<P>) -> Option<GameLoopError>
where
    P: AsRef<Path>,
{
    game_state
        .timer_thread
        .join()
        .map_err(GameLoopError::FailedToShutDown)
        .err()
}

#[derive(Debug, Error)]
pub enum GameLoopError {
    #[error("Failed to update game state. {0}")]
    UpdatingGameState(#[from] std::io::Error),
    #[error("Failed to render game state to terminal. {0}")]
    RenderingError(WindowRendererError),
    #[error("Failed to shut down game.")]
    FailedToShutDown(ThreadError),
}
