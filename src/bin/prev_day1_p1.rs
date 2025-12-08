use advent_of_code_2025::scanner::*;

fn main() {
    let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::File("input.txt".to_string()));

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in scan.into_iter() {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
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
