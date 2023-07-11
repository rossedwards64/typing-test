mod game;

use game::{game_logic, word_file};

static PATH: &str = "data/words.txt";

fn main() {
    let mut word_file = word_file::WordFile::new(PATH);
    let word = word_file.get_random_word();
    println!("{word}");
    game_logic::game();
}
