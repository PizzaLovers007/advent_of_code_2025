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

    let mut sum = 0;
    while let Some(line) = scan.try_spar() {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let n = digits.len();
        let mut left = 0;
        let mut ans: u64 = 0;
        for remain in (1..=12).rev() {
            ans *= 10;
            let right = n - remain;
            let (best_index, &best) = digits[left..=right]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|x| x.1)
                .unwrap();
            left += best_index + 1;
            ans += best as u64;
        }

        sum += ans;
    }

    println!("{sum}");
}
