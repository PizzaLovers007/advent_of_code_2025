use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut lines: Vec<String> = Vec::new();
    while let Some(line) = scan.try_spar() {
        lines.push(line.chars().collect());
    }

    let mut sum = 0;
    for line in lines {
        let n = line.len();
        let mut best = 0;
        for i in 0..n {
            for j in i + 1..n {
                let bat: i32 =
                    line[i..i + 1].parse::<i32>().unwrap() * 10 + line[j..j + 1].parse::<i32>().unwrap();
                best = bat.max(best);
            }
        }
        sum += best;
    }
    println!("{sum}");
}
