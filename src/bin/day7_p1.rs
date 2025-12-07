use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = ".......S.......
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

    let mut mat: Vec<Vec<char>> = scan
        .into_iter()
        .map(|line| String::from(line).chars().collect())
        .collect();

    let mut ans = 0;
    for r in 1..mat.len() {
        let mut row: Vec<char> = mat[r].clone();
        mat[r - 1]
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == '|' || c == 'S')
            .for_each(|(i, _)| {
                if r % 2 == 0 && row[i] == '^' {
                    row[i - 1] = '|';
                    row[i + 1] = '|';
                    ans += 1;
                } else {
                    row[i] = '|';
                }
            });
        mat[r] = row;
    }

    // for row in mat {
    //     println!("{}", row.iter().collect::<String>());
    // }

    println!("{ans}");
}
