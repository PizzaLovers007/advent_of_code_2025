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
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    let mut mat: Vec<Vec<char>> = Vec::new();
    while let Some(line) = scan.try_spar() {
        mat.push(line.chars().collect());
    }

    let n = mat.len();
    let m = mat[0].len();
    let dr: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
    let dc: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
    let mut ans = 0;
    loop {
        let mut mat_copy = mat.clone();
        let mut found = false;
        for r in 0i32..n as i32 {
            for c in 0i32..m as i32 {
                if mat[r as usize][c as usize] != '@' {
                    continue;
                }
                let mut count = 0;
                for (rr, cc) in dr.iter().map(|&x| r+x).zip(dc.iter().map(|&x| c+x)) {
                    let Ok(row) = usize::try_from(rr) else {
                        continue;
                    };
                    let Ok(col) = usize::try_from(cc) else {
                        continue;
                    };
                    if row < n && col < m && mat[row][col] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    mat_copy[r as usize][c as usize] = '.';
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
