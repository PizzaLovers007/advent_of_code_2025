use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut count = 0;
    while let Some(line) = scan.try_spar() {
        let mut levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut safe = false;
        for i in 0..levels.len() {
            let x = levels.remove(i);
            if is_safe(&levels) {
                safe = true;
                break;
            }
            levels.insert(i, x);
        }
        if safe {
            count += 1;
        }
    }

    println!("{count}");
}

fn is_safe(levels: &[i32]) -> bool {
    let mut all_increasing = true;
    let mut all_decreasing = true;
    let mut ok_diff = true;
    for i in 1..levels.len() {
        if levels[i-1] <= levels[i] {
            all_decreasing = false;
        }
        if levels[i-1] >= levels[i] {
            all_increasing = false;
        }
        let diff = (levels[i-1] - levels[i]).abs();
        if diff < 1 || diff > 3 {
            ok_diff = false;
        }
    }

    (all_increasing || all_decreasing) && ok_diff
}
