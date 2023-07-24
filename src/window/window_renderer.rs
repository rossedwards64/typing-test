use super::window::{Window, GameWindow, InfoWindow, InputWindow};

pub struct WindowRenderer {
    game_win: GameWindow,
    input_win: InputWindow,
    info_win: InfoWindow,
}

impl WindowRenderer {
    pub fn new() -> Self {
        Self {
            game_win: GameWindow::new(0, 0),
            input_win: InputWindow::new(0, 0),
            info_win: InfoWindow::new(0, 0),
        }
    }
}
