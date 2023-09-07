use std::path::Path;

use super::word_file::WordFile;

type WordList = Vec<Word>;

pub struct WordManager<P>
where
    P: AsRef<Path>,
{
    file: WordFile<P>,
    list: WordList,
    correct_words: u32,
    missed_words: u32,
}

impl<P> WordManager<P>
where
    P: AsRef<Path>,
{
    pub fn new(file: WordFile<P>) -> Self {
        Self {
            file,
            list: Vec::new(),
            correct_words: 0,
            missed_words: 0,
        }
    }

    pub fn spawn_word(&mut self) -> &Word {
        let word = Word::new(self.file.get_random_word());
        self.list.push(word);
        self.list.last().unwrap()
    }

    pub fn check_words() {}

    pub fn erase_words() {}

    const fn list(&self) -> &WordList {
        &self.list
    }
}

pub struct Word {
    chars: String,
    pos_x: u16,
    pos_y: u16,
    correct: bool,
}

impl Word {
    fn new(word: String) -> Self {
        Self {
            chars: word,
            pos_x: 0,
            pos_y: 0,
            correct: false,
        }
    }
}
