use super::word_file::WordFile;
use std::path::Path;

type WordList = Vec<Word>;

#[allow(dead_code)]
pub struct WordManager<P>
where
    P: AsRef<Path>,
{
    file: WordFile<P>,
    list: WordList,
    correct_words: u32,
    missed_words: u32,
}

#[allow(dead_code)]
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

    pub const fn check_words() {}

    pub const fn erase_words() {}

    const fn list(&self) -> &WordList {
        &self.list
    }
}

#[allow(dead_code)]
pub struct Word {
    chars: String,
    pos_x: u16,
    pos_y: u16,
    correct: bool,
}

impl Word {
    const fn new(word: String) -> Self {
        Self {
            chars: word,
            pos_x: 0,
            pos_y: 0,
            correct: false,
        }
    }
}
