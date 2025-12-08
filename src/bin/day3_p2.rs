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

    let mut sum: i64 = 0;
    for line in lines {
        let n = line.len();
        let mut dp: Vec<Vec<i64>> = vec![vec![-1; 13]; n + 1];
        let arr = line
            .chars()
            .map(|c| i64::from(c.to_digit(10).unwrap()))
            .collect();
        sum += solve(&mut dp, &arr, 0, 12);
    }
    println!("{sum}");
}

fn solve(dp: &mut Vec<Vec<i64>>, arr: &Vec<i64>, i: usize, remain: usize) -> i64 {
    if remain == 0 {
        return 0;
    }
    if i == arr.len() {
        return -1000000000000;
    }
    if dp[i][remain] != -1 {
        return dp[i][remain];
    }
    let one = solve(dp, arr, i + 1, remain);
    let two = arr[i] * i64::from(10).pow(u32::try_from(remain - 1).unwrap())
        + solve(dp, arr, i + 1, remain - 1);
    let best = one.max(two);
    dp[i][remain] = best;

    best
}
