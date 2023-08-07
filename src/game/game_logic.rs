use super::word_manager::WordManager;
use crate::window::window_renderer::WindowRenderer;

struct Game<'a> {
    word_manager: WordManager,
    renderer: WindowRenderer<'a>,
}

pub fn game_loop() {}
