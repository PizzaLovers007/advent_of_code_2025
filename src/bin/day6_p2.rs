use std::{fs::File, io::Read};

use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    // let mut scan = Scanner::new(ScannerSource::File("input.txt"));

    scan.set_delimiter('\n');

    let file = File::open("input.txt").expect("Unable to open file");
    let contents: String = file
        .bytes()
        .map(|res_b| res_b.expect("Read error") as char)
        .collect();
    let line_chars: Vec<Vec<char>> = contents
        .lines()
        .map(|line| String::from(line).chars().collect())
        .collect();

    let n = line_chars.len();
    let m = line_chars[0].len();
    let mut ans = 0;
    let mut op = ' ';
    let mut nums: Vec<_> = Vec::new();
    for j in (0..m).rev() {
        let mut num = 0u64;
        for i in 0..n {
            if let Some(digit) = line_chars[i][j].to_digit(10) {
                num *= 10;
                num += digit as u64;
            } else if line_chars[i][j] != ' ' {
                op = line_chars[i][j];
            }
        }
        if num > 0 {
            // println!("{num}");
            nums.push(num);
        }
        match op {
            '+' => {
                let sum = nums.iter().sum::<u64>();
                ans += sum;
                // println!("+ {sum}");
                nums.clear();
            }
            '*' => {
                let product = nums.iter().product::<u64>();
                ans += product;
                // println!("* {product}");
                nums.clear();
            }
            _ => (),
        }
        op = ' ';
    }

    println!("{ans}");
}
