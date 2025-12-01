use advent_of_code_2025::scanner::*;

fn main() {
    // let mut scan = Scanner::new(ScannerSource::Stdin);
    let mut scan = Scanner::new(ScannerSource::File("input.txt".to_string()));
    
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
