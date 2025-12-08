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

    let mut ranges: Vec<_> = Vec::new();
    let mut state = 0;
    let mut ans = 0;
    for line in scan.into_iter() {
        let s = String::from(line);
        if s.is_empty() {
            state = 1;
            println!("{ranges:?}");
            continue;
        }
        if state == 0 {
            let (left, right) = s.split_once('-').unwrap();
            ranges.push(left.parse::<i64>().unwrap()..right.parse::<i64>().unwrap()+1);
        } else {
            let x = s.parse().unwrap();
            for r in ranges.iter() {
                if r.contains(&x) {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{ans}");
}
