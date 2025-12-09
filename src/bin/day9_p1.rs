use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let lines: Vec<(i64, i64)> = scan
        .into_iter()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let n = lines.len();

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let (x1, y1) = lines[i];
            let (x2, y2) = lines[j];
            ans = ans.max((x1-x2+1).abs() * (y1-y2+1).abs());
        }
    }

    println!("{ans}");
}
