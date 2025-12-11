use std::collections::HashMap;

use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

struct Node {
    id: String,
    others: Vec<String>,
}

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let lines: Vec<_> = scan.into_iter().collect();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let Some((id, rest)) = line.split_once(':') else {
            panic!();
        };

        let other_ids: Vec<_> = rest.trim().split(' ').collect();
        let curr = nodes.entry(id.to_string()).or_insert(Node {
            id: id.to_string(),
            others: Vec::new(),
        });

        for other_id in other_ids {
            curr.others.push(other_id.to_string());
        }
    }

    let ans = brute_force(&nodes, "you");
    println!("{ans}");
}

fn brute_force(nodes: &HashMap<String, Node>, curr_id: &str) -> i32 {
    if curr_id == "out" {
        return 1;
    }
    let curr = nodes.get(curr_id).unwrap();
    let mut sum = 0;
    for other_id in &curr.others {
        sum += brute_force(nodes, other_id);
    }
    
    sum
}
