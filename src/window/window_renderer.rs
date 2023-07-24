use super::window::{InputWindow, GameWindow, InfoWindow};

pub struct WindowRenderer {
    game_win: GameWindow,
    input_win: InputWindowm,
    info_win: InfoWindow
}

impl WindowRenderer {
    pub fn new() -> Self {
        Self {
            game_win: GameWindow::new(),
            input_win: InputWindow::new(),
            info_win: InfoWindow::new(),
        }
    }
}
