use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, Read};

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

pub enum ScannerSource<'a> {
    Stdin,
    File(&'a str),
    Constant(&'a str),
}

pub struct Scanner<'a> {
    source: ScannerSource<'a>,
    buffer: VecDeque<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: ScannerSource<'a>) -> Self {
        let mut scan = Scanner {
            source,
            buffer: VecDeque::new(),
        };

        match &scan.source {
            ScannerSource::Stdin => {
                scan.buffer = std::io::stdin()
                    .lock()
                    .lines()
                    .map(|s| s.unwrap())
                    .skip_while(|line| line.is_empty())
                    .collect();
            }
            ScannerSource::File(path) => {
                let file = File::open(path).expect("Unable to open file");
                let contents: String = file
                    .bytes()
                    .map(|res_b| res_b.expect("Read error") as char)
                    .collect();
                scan.buffer = contents
                    .lines()
                    .skip_while(|&line| line.is_empty())
                    .map(|s| s.to_string())
                    .collect();
            }
            ScannerSource::Constant(s) => {
                scan.buffer = s
                    .lines()
                    .skip_while(|&line| line.is_empty())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        scan
    }

    pub fn par<T: From<ScannerToken>>(&mut self) -> T {
        ScannerToken(self.buffer.pop_front().expect("No token")).into()
    }

    pub fn ipar(&mut self) -> i32 {
        self.par()
    }

    pub fn lpar(&mut self) -> i64 {
        self.par()
    }

    pub fn upar(&mut self) -> usize {
        self.par()
    }

    pub fn dpar(&mut self) -> f64 {
        self.par()
    }

    pub fn spar(&mut self) -> String {
        self.par()
    }

    pub fn apar<T: From<ScannerToken>>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.par()).collect()
    }

    pub fn iapar(&mut self, n: usize) -> Vec<i32> {
        self.apar(n)
    }

    pub fn lapar(&mut self, n: usize) -> Vec<i64> {
        self.apar(n)
    }

    pub fn uapar(&mut self, n: usize) -> Vec<usize> {
        self.apar(n)
    }

    pub fn sapar(&mut self, n: usize) -> Vec<String> {
        self.apar(n)
    }

    pub fn try_par<T: From<ScannerToken>>(&mut self) -> Option<T> {
        self.buffer.pop_front().map(ScannerToken).map(|x| x.into())
    }

    pub fn try_ipar(&mut self) -> Option<i32> {
        self.try_par()
    }

    pub fn try_lpar(&mut self) -> Option<i64> {
        self.try_par()
    }

    pub fn try_upar(&mut self) -> Option<usize> {
        self.try_par()
    }

    pub fn try_dpar(&mut self) -> Option<f64> {
        self.try_par()
    }

    pub fn try_spar(&mut self) -> Option<String> {
        self.try_par()
    }
}

impl<'a> IntoIterator for Scanner<'a> {
    type Item = ScannerToken;
    type IntoIter = ScannerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ScannerIterator { scan: self }
    }
}

pub struct ScannerIterator<'a> {
    scan: Scanner<'a>,
}

impl<'a> Iterator for ScannerIterator<'a> {
    type Item = ScannerToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan.try_par()
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
