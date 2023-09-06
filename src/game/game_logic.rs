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
    terminate: bool,
}

pub fn game_loop<P>(file: WordFile<P>, renderer: WindowRenderer<'_>)
where
    P: AsRef<Path>,
{
    let mut game_state = GameState {
        word_manager: WordManager::new(file),
        renderer,
        terminate: false,
    };

    loop {
        update(&mut game_state);
        input(&mut game_state.terminate);
        render(&mut game_state.renderer);

        if game_state.terminate {
            break;
        }
    }
}

fn update<P>(game_state: &mut GameState<P>)
where
    P: AsRef<Path>,
{
    game_state.word_manager.spawn_word();
}

fn input(terminate: &mut bool) -> String {
    let mut buf = String::new();
    if matches!(event::poll(Duration::from_millis(100)), Ok(true)) {
        while let Ok(event) = event::read() {
            if let event::Event::Key(key) = event {
                match key.code {
                    event::KeyCode::Char(c) => buf.push(c),
                    event::KeyCode::Backspace => {
                        buf.pop();
                    }
                    event::KeyCode::Esc => {
                        *terminate = true;
                    }
                    _ => (),
                }
            }
        }
    }

    buf
}

fn render(renderer: &mut WindowRenderer) {
    renderer.render_windows();
}
