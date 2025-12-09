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

    let points: Vec<(i64, i64)> = scan
        .into_iter()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let n = points.len();

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let (mut x1, mut y1) = points[i];
            let (mut x2, mut y2) = points[j];
            (x1, x2) = (x1.min(x2), x1.max(x2));
            (y1, y2) = (y1.min(y2), y1.max(y2));
            let mut intersect = false;
            for k in 0..n {
                let (mut x3, mut y3) = points[k];
                let (mut x4, mut y4) = points[(k+1)%n];
                (x3, x4) = (x3.min(x4), x3.max(x4));
                (y3, y4) = (y3.min(y4), y3.max(y4));
                if x3 == x4 {
                    intersect |= x1 == x3 && (y1 < y3 && y3 < y2 || y1 < y4 && y4 < y2);
                    intersect |= x2 == x3 && (y1 < y3 && y3 < y2 || y1 < y4 && y4 < y2);
                    intersect |= x1 < x3 && x3 < x2 && y3 < y1 && y1 < y4;
                    intersect |= x1 < x3 && x3 < x2 && y3 < y2 && y2 < y4;
                } else if y3 == y4 {
                    intersect |= y1 == y3 && (x1 < x3 && x3 < x2 || x1 < x4 && x4 < x2);
                    intersect |= y2 == y3 && (x1 < x3 && x3 < x2 || x1 < x4 && x4 < x2);
                    intersect |= y1 < y3 && y3 < y2 && x3 < x1 && x1 < x4;
                    intersect |= y1 < y3 && y3 < y2 && x3 < x2 && x2 < x4;
                }
                if intersect {
                    break;
                }
            }
            if !intersect {
                ans = ans.max((x2-x1+1) * (y2-y1+1));
            }
        }
    }

    println!("{ans}");
}
