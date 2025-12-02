use advent_of_code_2025::scanner::*;

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    let mut scan = Scanner::new(ScannerSource::File("input.txt"));
    
    let mut curr = 50;
    let mut count = 0;
    while let Some(command) = scan.try_par::<String>() {
        let mut it = command.chars();
        let dir = it.next().unwrap();
        let mut num = it.collect::<String>().parse::<i32>().unwrap();
        count += num / 100;
        num %= 100;
        for _ in 0..num {
            if dir == 'L' {
                curr -= 1;
            } else {
                curr += 1;
            }
            if curr % 100 == 0 {
                count += 1;
            }
        }
    }
    println!("{count}");
}
