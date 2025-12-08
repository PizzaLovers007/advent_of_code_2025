use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    // let mut scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));
    
    let mut curr = 50;
    let mut count = 0;
    while let Some(command) = scan.try_par::<String>() {
        let mut it = command.chars();
        let dir = it.next().unwrap();
        let mut num = it.collect::<String>().parse::<i32>().unwrap();
        if dir == 'L' {
            num *= -1;
        }
        curr += num + 100;
        curr %= 100;
        if curr == 0 {
            count += 1;
        }
    }
    println!("{count}");
}
