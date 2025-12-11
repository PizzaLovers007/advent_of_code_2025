use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    out_degree: usize,
    out_count: usize,
    fft_count: usize,
    dac_count: usize,
    others: Vec<&'a str>,
}

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let lines: Vec<_> = scan.into_iter().collect();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut parent_graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.iter() {
        let Some((id, rest)) = line.split_once(':') else {
            panic!();
        };

        let other_ids: Vec<_> = rest.trim().split(' ').collect();
        let curr = nodes.entry(id).or_insert(Node {
            id,
            out_degree: other_ids.len(),
            out_count: 0,
            fft_count: 0,
            dac_count: 0,
            others: Vec::new(),
        });

        if id == "fft" {
            curr.fft_count = 1;
        } else if id == "dac" {
            curr.dac_count = 1;
        }

        for other_id in other_ids {
            let parents = parent_graph.entry(other_id).or_insert(Vec::new());
            parents.push(id);
            curr.others.push(other_id);
        }
    }

    nodes.insert(
        "out",
        Node {
            id: "out",
            out_degree: 0,
            out_count: 1,
            fft_count: 0,
            dac_count: 0,
            others: Vec::new(),
        },
    );

    let mut queue = VecDeque::new();
    for node in nodes.values() {
        if node.out_degree == 0 {
            queue.push_back(node.id);
        }
    }

    for node in parent_graph.iter() {
        println!("{node:?}");
    }

    while let Some(curr_id) = queue.pop_front() {
        let curr = nodes.get(curr_id).unwrap();
        let Some(parents) = parent_graph.get(curr.id) else {
            continue;
        };
        let mut nodes_to_add: Vec<&str> = Vec::new();
        for &parent in parents {
            nodes.entry(parent).and_modify(|e| {
                e.out_degree -= 1;
                if e.out_degree == 0 {
                    nodes_to_add.push(e.id);
                }
            });
        }
        for node_id in nodes_to_add {
            let Some(node) = nodes.get(node_id) else {
                continue;
            };
            queue.push_back(node_id);
            let mut out_count = 0;
            let mut fft_count = 0;
            let mut dac_count = 0;
            for &other_id in &node.others {
                let Some(other) = nodes.get(other_id) else {
                    continue;
                };
                out_count += other.out_count;
                fft_count += other.fft_count;
                dac_count += other.dac_count;
            }
            let Some(node) = nodes.get_mut(node_id) else {
                continue;
            };
            node.out_count = out_count;
            node.fft_count = node.fft_count.max(fft_count);
            node.dac_count = node.dac_count.max(dac_count);
        }
    }

    let a1 = nodes.get("svr").unwrap().fft_count;
    let a2 = nodes.get("fft").unwrap().dac_count;
    let a3 = nodes.get("dac").unwrap().out_count;
    let b1 = nodes.get("svr").unwrap().dac_count;
    let b2 = nodes.get("dac").unwrap().fft_count;
    let b3 = nodes.get("fft").unwrap().out_count;
    let ans = (a1*a2*a3).max(b1*b2*b3);

    for node in nodes.values() {
        println!("{node:?}");
    }

    println!("{ans}");
}
