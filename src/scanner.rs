use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub struct ScannerToken(String);

impl From<ScannerToken> for i32 {
    fn from(value: ScannerToken) -> Self {
        value.0.parse().unwrap()
    }
}

impl From<ScannerToken> for i64 {
    fn from(value: ScannerToken) -> Self {
        value.0.parse().unwrap()
    }
}

impl From<ScannerToken> for usize {
    fn from(value: ScannerToken) -> Self {
        value.0.parse().unwrap()
    }
}

impl From<ScannerToken> for f64 {
    fn from(value: ScannerToken) -> Self {
        value.0.parse().unwrap()
    }
}

impl From<ScannerToken> for String {
    fn from(value: ScannerToken) -> Self {
        value.0
    }
}

pub enum ScannerSource {
    Stdin,
    File(String),
}

pub struct Scanner {
    source: ScannerSource,
    buffer: VecDeque<String>,
    finished: bool,
}

impl Scanner {
    pub fn new(source: ScannerSource) -> Self {
        Scanner {
            source,
            buffer: VecDeque::new(),
            finished: false,
        }
    }

    fn fill_buffer(&mut self) {
        if self.finished {
            return;
        }

        match &self.source {
            ScannerSource::Stdin => {
                let stream = std::io::stdin()
                    .lock()
                    .bytes()
                    .map(|res_b| res_b.expect("Read error"))
                    .map(char::from);
                let word: String = stream
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| !c.is_whitespace())
                    .collect();
                if word.is_empty() {
                    self.finished = true;
                } else {
                    self.buffer.push_back(word);
                }
            }
            ScannerSource::File(path) => {
                let file = File::open(path).expect("Unable to open file");
                let contents: String = file
                    .bytes()
                    .map(|res_b| res_b.expect("Read error"))
                    .map(char::from)
                    .collect();
                self.buffer = contents.split_whitespace().map(|s| s.to_string()).collect();
                self.finished = true;
            }
        }
    }

    pub fn par<T: From<ScannerToken>>(&mut self) -> T {
        self.fill_buffer();
        ScannerToken(self.buffer.pop_front().expect("No token")).into()
    }

    pub fn apar<T: From<ScannerToken>>(&mut self, n: usize) -> Vec<T> {
        self.fill_buffer();
        (0..n).map(|_| self.par()).collect()
    }

    pub fn try_par<T: From<ScannerToken>>(&mut self) -> Option<T> {
        self.fill_buffer();
        self.buffer.pop_front().map(ScannerToken).map(|x| x.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_token_parses() {
        let num: i32 = ScannerToken(String::from("2")).into();
        assert_eq!(num, 2);
    }
}
