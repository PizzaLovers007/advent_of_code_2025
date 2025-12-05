use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut mat: Vec<Vec<char>> = scan
        .into_iter()
        .map(|t| String::from(t).chars().collect())
        .collect();

    let n = mat.len();
    let m = mat[0].len();
    let dr: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
    let dc: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
    let deltas: Vec<_> = dr.into_iter().zip(dc.into_iter()).collect();
    let mut ans = 0;
    loop {
        let mut mat_copy = mat.clone();
        let mut found = false;
        for r in 0..n {
            for c in 0..m {
                if mat[r][c] != '@' {
                    continue;
                }
                let count = deltas
                    .iter()
                    .map(|&(dr, dc)| (r as i32 + dr, c as i32 + dc))
                    .filter(|&(row, col)| {
                        (0..n as i32).contains(&row) && (0..m as i32).contains(&col)
                    })
                    .map(|(row, col)| mat[row as usize][col as usize])
                    .filter(|&c| c == '@')
                    .count();
                if count < 4 {
                    mat_copy[r][c] = '.';
                    ans += 1;
                    found = true;
                }
            }
        }
        mat = mat_copy;
        if !found {
            break;
        }
    }
    println!("{ans}");
}
