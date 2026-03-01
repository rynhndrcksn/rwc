pub struct Counter {
    pub bytes: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl Counter {
    pub fn from_bytes(raw: &[u8]) -> Self {
        let text = String::from_utf8_lossy(raw);
        Self {
            bytes: raw.len(),
            lines: text.lines().count(),
            words: text.split_whitespace().count(),
            chars: text.chars().count(),
        }
    }
}