use advent_of_code_2025::scanner::*;

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

        // Buttons as a bitmask for which lights they affect
        let buttons: Vec<usize> = parts[1..parts.len() - 1]
            .iter()
            .map(|&p| {
                p[1..p.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .fold(0, |acc, x| acc | (1 << x))
            })
            .collect();

        // Joltage requirements
        let mut jolts: Vec<i64> = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        /*
        Observation 1: In the optimal solution, every button will be pressed
        either an even or an odd number of times.

        Observation 2: A button pressed an odd number of times consists of a
        single press followed by an even number of presses.

        So the optimal solution is split into two parts:
        1. Some combination of single-presses.
        2. Some combination of even-presses.

        Observation 3: Step 2's joltage requirements will always be even.

        If we halve the step 2 joltage requirements, this results in a
        subproblem we can recursively solve for!
        */

        // Calculate all feasible light states. Keep track of button presses.
        let mut dp = vec![Vec::new(); 1 << jolts.len()];
        for button_mask in 0..1usize << buttons.len() {
            let mut state = 0;
            for i in 0..buttons.len() {
                if button_mask & (1 << i) != 0 {
                    state ^= buttons[i];
                }
            }
            dp[state].push(button_mask);
        }

        let best = solve(&mut jolts, &buttons, &dp);
        ans += best;
    }

    println!("{ans}");
}

fn solve(jolts: &mut Vec<i64>, buttons: &Vec<usize>, dp: &Vec<Vec<usize>>) -> u32 {
    if jolts.iter().all(|&x| x == 0) {
        return 0;
    }

    // Calculate light state for step 1
    let mut light_state = 0;
    for j in 0..jolts.len() {
        light_state |= (1 << j) * (jolts[j] % 2) as usize;
    }

    // Iterate through each way to achieve that light state
    let mut best = 1000000;
    for button_mask in &dp[light_state] {
        // Update joltages with the given button presses
        let mut possible = true;
        for i in 0..buttons.len() {
            if button_mask & (1 << i) != 0 {
                for j in 0..jolts.len() {
                    jolts[j] -= (buttons[i] >> j) as i64 & 1;
                    if jolts[j] < 0 {
                        possible = false;
                    }
                }
            }
        }

        // For valid states, recurse down with remaining joltage requirements
        if possible {
            for j in 0..jolts.len() {
                jolts[j] /= 2;
            }
            best = best.min(solve(jolts, buttons, dp) * 2 + button_mask.count_ones());
            for j in 0..jolts.len() {
                jolts[j] *= 2;
            }
        }

        // Revert joltage updates
        for i in 0..buttons.len() {
            if button_mask & (1 << i) != 0 {
                for j in 0..jolts.len() {
                    jolts[j] += (buttons[i] >> j) as i64 & 1;
                }
            }
        }
    }

    best
}
