use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let lines: Vec<_> = scan.into_iter().collect();

    let mut ans = 0;
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let state: Vec<_> = parts[0][1..parts[0].len() - 1].chars().collect();

        let switches: Vec<i64> = parts[1..parts.len() - 1]
            .iter()
            .map(|&p| {
                p[1..p.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .fold(0, |acc, x| acc | (1 << x))
            })
            .collect();
        
        let mut best = 10000;
        for mask in 0..1<<switches.len() {
            let mut bits = 0;
            let mut count = 0;
            for i in 0..switches.len() {
                if mask & (1 << i) != 0 {
                    bits ^= switches[i];
                    count += 1;
                }
            }
            let mut good = true;
            for i in 0..state.len() {
                if (state[i] == '#') != (bits & (1 << i) != 0) {
                    good = false;
                    break;
                }
            }
            if good {
                best = best.min(count);
            }
        }

        ans += best;
    }

    println!("{ans}");
}
