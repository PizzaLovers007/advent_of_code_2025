use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    scan.set_delimiter('\n');

    let mut nums: Vec<Vec<_>> = Vec::new();
    let mut ops: Vec<_> = Vec::new();
    while let Some(line) = scan.try_spar() {
        let tokens: Vec<_> = line.split_whitespace().map(|t| t.to_string()).collect();
        if tokens[0].chars().next().unwrap().is_digit(10) {
            nums.push(tokens.iter().map(|t| t.parse::<i64>().unwrap()).collect());
        } else {
            ops = tokens;
        }
    }

    // println!("{nums:?}");
    // println!("{ops:?}");

    let mut ans = 0;
    for i in 0..nums[0].len() {
        let op = ops[i].as_str();
        let mut subans: i64;
        match op {
            "+" => subans = 0,
            "*" => subans = 1,
            _ => panic!(),
        }
        for j in 0..nums.len() {
            match op {
                "+" => subans += nums[j][i],
                "*" => subans *= nums[j][i],
                _ => panic!(),
            }
        }
        ans += subans;
    }
    println!("{ans}");
}
