use advent_of_code_2025::scanner::*;

fn main() {
    let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::File("input.txt".to_string()));

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    while let (Some(l), Some(r)) = (scan.try_par(), scan.try_par()) {
        left.push(l);
        right.push(r);
    }

    let n = left.len();
    left.sort();
    right.sort();
    let mut dist = 0;
    for i in 0..n {
        dist += (left[i] - right[i]).abs();
    }
    println!("{dist}");
}
