type WordList = Vec<Word>;

pub(super) struct WordManager {
    list: WordList,
    correct_words: u32,
    missed_words: u32,
}

impl WordManager {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            correct_words: 0,
            missed_words: 0,
        }
    }

    fn spawn_word() {}

    fn check_words() {}

    fn erase_words() {}

    const fn list(&self) -> &WordList {
        &self.list
    }
}

pub(self) struct Word {
    chars: String,
    pos_x: u16,
    pos_y: u16,
    correct: bool,
}

impl Word {
    fn new() -> Self {
        Self {
            chars: String::new(),
            pos_x: 0,
            pos_y: 0,
            correct: false,
        }
    }
}
