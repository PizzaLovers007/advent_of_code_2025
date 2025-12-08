use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut lefts: Vec<_> = Vec::new();
    let mut rights: Vec<_> = Vec::new();
    for s in scan.into_iter() {
        if s.is_empty() {
            break;
        }
        let (left, right) = s.split_once('-').unwrap();
        lefts.push(left.parse::<i64>().unwrap());
        rights.push(right.parse::<i64>().unwrap());
    }
    lefts.sort();
    rights.sort();
    let mut ans = 0;
    let mut count = 0;
    let mut l = 0;
    let mut r = 0;
    let mut prev = -1;
    while l < lefts.len() && r < rights.len() {
        let left = lefts[l];
        let right = rights[r];
        if left <= right {
            count += 1;
            l += 1;
            if count == 1 {
                prev = left;
            }
        } else {
            count -= 1;
            r += 1;
            if count == 0 {
                ans += right - prev + 1;
            }
        }
        // println!("{left} {right} {count} {ans}");
    }
    while r < rights.len() {
        let right = rights[r];
        let curr = right;
        count -= 1;
        r += 1;
        if count == 0 {
            ans += curr - prev + 1;
        }
        // println!("--- {right} {count} {ans}");
    }
    println!("{ans}");
}
