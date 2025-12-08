use std::collections::HashMap;

use advent_of_code_2025::scanner::*;

fn main() {
    let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::File("input.txt".to_string()));

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in scan.into_iter() {
        let mut iter = line.split_whitespace();
        let l = iter.next().unwrap().parse().unwrap();
        let r = iter.next().unwrap().parse().unwrap();
        left.push(l);
        right.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut similar = 0;
    for l in left {
        if let Some(count) = right.get(&l) {
            similar += l * count;
        }
    }
    println!("{similar}");
}
