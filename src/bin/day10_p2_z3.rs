use advent_of_code_2025::scanner::*;
use z3::{Optimize, ast::Int};

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

        let mut switches: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|&p| {
                p[1..p.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        switches.sort_by_key(|l| -(l.len() as i32));

        let jolts: Vec<i32> = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let optimize = Optimize::new();

        let mut vars = Vec::new();
        for i in 0..switches.len() {
            vars.push(Int::new_const(format!("x{i}")));
            optimize.assert(&vars[i].ge(0));
        }

        for (index, &jolt) in jolts.iter().enumerate() {
            let affected_vars: Vec<_> = (0..switches.len())
                .filter(|&i| switches[i].contains(&index))
                .map(|i| &vars[i])
                .collect();
            optimize.assert(&Int::add(&affected_vars).eq(jolt));
        }

        optimize.minimize(&Int::add(&vars));

        _ = optimize.check(&[]);
        let model = optimize.get_model().unwrap();

        let mut best = 0;
        for v in vars {
            best += model.eval(&v, true).unwrap().as_i64().unwrap();
        }

        // println!("{best}");
        ans += best;
    }

    println!("{ans}");
}
