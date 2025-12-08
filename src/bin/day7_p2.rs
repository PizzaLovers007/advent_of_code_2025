use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let mat: Vec<Vec<char>> = scan
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let n = mat.len();
    let m = mat[0].len();

    let mut dp = vec![vec![1i64; m]; n];

    for c in 0..m {
        if mat[n - 2][c] == '^' {
            dp[n - 2][c] = 2;
        }
    }

    for r in (2..n-2).rev().filter(|&i| i % 2 == 0) {
        for c in 0..m {
            if mat[r][c] == '^' {
                dp[r][c] = 2;
                for r2 in (r+2..n).filter(|&i| i % 2 == 0) {
                    if mat[r2][c-1] == '^' {
                        dp[r][c] += dp[r2][c-1] - 1;
                        break;
                    }
                }
                for r2 in (r+2..n).filter(|&i| i % 2 == 0) {
                    if mat[r2][c+1] == '^' {
                        dp[r][c] += dp[r2][c+1] - 1;
                        break;
                    }
                }
            }
        }
    }

    let start_c = mat[0]
        .iter()
        .enumerate()
        .find(|&(_, &ch)| ch == 'S')
        .expect("No start")
        .0;

    // for arr in dp.iter() {
    //     println!("{arr:?}");
    // }

    println!("{}", dp[2][start_c]);
}
