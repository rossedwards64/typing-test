pub mod game;
pub mod window;

#[cfg(test)]
mod tests {
    use super::game::word_file;

    #[test]
    fn test() {
        let words = "data/words.txt";
        let word_file = word_file::WordFile::new(&words);
        assert_eq!(word_file.len(), 10000);
    }
}
