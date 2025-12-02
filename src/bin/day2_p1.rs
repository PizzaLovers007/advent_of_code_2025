use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));
    scan.set_delimiter(',');

    let mut sum: i64 = 0;
    while let Some(key) = scan.try_spar() {
        let (s1, s2) = key.split_once('-').unwrap();
        let first: i64 = s1.parse().unwrap();
        let last: i64 = s2.parse().unwrap();
        for x in first..last+1 {
            if !is_valid(&format!("{x}")) {
                sum += x;
                // println!("{x}");
            }
        }
    }

    println!("{sum}");
}

fn is_valid(s: &str) -> bool {
    if s.len()%2 == 1 {
        return true;
    }
    let n = s.len();
    let a = &s[0..n/2];
    let b = &s[n/2..n];
    return a != b;
}
