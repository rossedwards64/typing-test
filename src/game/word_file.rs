use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use rand::{rngs::ThreadRng, Rng};

pub struct WordFile<P>
where
    P: AsRef<Path>,
{
    path: P,
    len: usize,
    rng: ThreadRng,
}

impl<P> WordFile<P>
where
    P: AsRef<Path>,
{
    pub fn new(path: P) -> Self {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(err) => std::panic!("Error opening file! Reason: {err}"),
        };

        let reader = BufReader::new(&file);

        Self {
            path,
            len: reader.lines().count(),
            rng: rand::thread_rng(),
        }
    }

    pub fn get_random_word(&mut self) -> String {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(err) => std::panic!("Error opening file! Reason: {err}"),
        };

        let word_to_read = self.rng.gen_range(0..self.len);
        let mut reader = BufReader::new(&file);
        let mut buf = String::new();
        let mut cur_line = 0;

        while let Ok(s) = reader.read_line(&mut buf) {
            if s == 0 || cur_line == word_to_read {
                break;
            }
            cur_line += 1;
            buf.clear();
        }
        println!("{word_to_read}: {buf}");
        buf
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
}
