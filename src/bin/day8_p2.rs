use std::mem;

use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[derive(Debug)]
struct Point(i64, i64, i64);

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl FromIterator<String> for Point {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
            return Point(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        }
        panic!("Invalid format");
    }
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, mut i: usize, mut j: usize) -> bool {
        i = self.find(i);
        j = self.find(j);
        if i == j {
            return false;
        }
        if self.size[i] < self.size[j] {
            mem::swap(&mut i, &mut j);
        }
        self.parent[j] = i;
        self.size[i] += self.size[j];
        true
    }
}

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let points: Vec<Point> = scan
        .into_iter()
        .map(|t| String::from(t).split(',').map(|s| s.to_string()).collect())
        .collect();

    let n = points.len();
    let mut dist: Vec<Vec<i64>> = vec![vec![0; n]; n];
    let mut sorted_dist: Vec<(usize, usize, i64)> = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            dist[i][j] = points[i].distance(&points[j]);
            sorted_dist.push((i, j, dist[i][j]));
        }
    }
    sorted_dist.sort_by_key(|x| x.2);
    // println!("{sorted_dist:?}");

    let mut dsu = DisjointSet::new(n);
    let mut merged = 0;
    for &(i, j, _) in sorted_dist.iter() {
        if dsu.union(i, j) {
            merged += 1;
            if merged == n-1 {
                println!("{}", points[i].0 * points[j].0);
                break;
            }
        }
    }
}
