use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut sum: i64 = 0;
    for key in scan.spar().split(',') {
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
    for n in 1..s.len() {
        if s.len() % n != 0 {
            continue;
        }
        let slices: Vec<&str> = (0..s.len()/n).map(|i| &s[i*n..(i+1)*n]).collect();
        let mut diff = false;
        for i in 0..slices.len() {
            if slices[i] != slices[0] {
                diff = true;
                break;
            }
        }
        if !diff {
            return false;
        }
    }
    return true;
}
