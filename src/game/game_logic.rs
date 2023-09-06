use std::{path::Path, time::Duration};

use crossterm::event;

use super::{word_file::WordFile, word_manager::WordManager};
use crate::window::window_renderer::WindowRenderer;

struct GameState<'a, P>
where
    P: AsRef<Path>,
{
    word_manager: WordManager<P>,
    renderer: WindowRenderer<'a>,
    input_buf: String,
    terminate: bool,
}

pub fn game_loop<P>(file: WordFile<P>, renderer: WindowRenderer<'_>)
where
    P: AsRef<Path>,
{
    let mut game_state = GameState {
        word_manager: WordManager::new(file),
        renderer,
        input_buf: String::new(),
        terminate: false,
    };

    loop {
        update(&mut game_state);
        input(&mut game_state);
        render(&mut game_state);

        if game_state.terminate {
            return;
        }
    }
}

fn update<P>(game_state: &mut GameState<P>)
where
    P: AsRef<Path>,
{
    game_state.word_manager.spawn_word();
}

fn input<P>(game_state: &mut GameState<P>)
where
    P: AsRef<Path>,
{
    if matches!(event::poll(Duration::from_millis(100)), Ok(true)) {
        if let Ok(event) = event::read() {
            if let event::Event::Key(key) = event {
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
    }
}

fn render<P>(game_state: &mut GameState<P>)
where
    P: AsRef<Path>,
{
    game_state.renderer.render_windows(&game_state.input_buf);
}
