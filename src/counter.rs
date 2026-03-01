use std::io;
use std::io::{BufRead, BufReader, Read};

pub struct Counter {
    pub bytes: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl Counter {
    /// Reads the input in chunks, rather than all at once.
    pub fn from_reader<R: Read>(reader: R) -> io::Result<Self> {
        let reader = BufReader::new(reader);
        let mut bytes = 0;
        let mut lines = 0;
        let mut words = 0;
        let mut chars = 0;

        for line in reader.lines() {
            let line = line?;
            lines += 1;
            bytes += line.len() + 1;
            words += line.split_whitespace().count();
            chars += line.chars().count() + 1;
        }

        Ok(Self { bytes, lines, words, chars })
    }
}