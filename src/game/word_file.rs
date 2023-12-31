use rand::{rngs::ThreadRng, Rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct WordFile<P>
where
    P: AsRef<Path>,
{
    path: P,
    len: usize,
    rng: ThreadRng,
}

#[allow(dead_code)]
impl<P> WordFile<P>
where
    P: AsRef<Path>,
{
    pub fn new(path: P) -> Self {
        let file = open_file(&path);

        Self {
            path,
            len: BufReader::new(&file).lines().count(),
            rng: rand::thread_rng(),
        }
    }

    pub fn get_random_word(&mut self) -> String {
        let file = open_file(&self.path);
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
        buf
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path>,
{
    match File::open(path) {
        Ok(f) => f,
        Err(err) => std::panic!("Error opening file! Reason: {err}"),
    }
}
