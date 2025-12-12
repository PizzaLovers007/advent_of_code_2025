use advent_of_code_2025::scanner::*;

#[allow(dead_code)]
const SAMPLE: &str = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(Debug)]
struct Present {
    tile_count: usize,
    shapes: Vec<Vec<Vec<bool>>>,
}

fn main() {
    // let scan = Scanner::new(ScannerSource::Stdin);
    // let scan = Scanner::new(ScannerSource::Constant(SAMPLE));
    let scan = Scanner::new(ScannerSource::File("input.txt"));

    let lines: Vec<_> = scan.into_iter().collect();

    let mut presents: Vec<Present> = Vec::new();

    for i in 0..6 {
        let start = i * 5 + 1;
        let mut shape: Vec<Vec<_>> = lines[start..start + 3]
            .iter()
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect();
        let present = Present {
            shapes: Vec::new(),
            tile_count: shape.iter().flat_map(|row| row.iter()).filter(|&&b| b).count(),
        };
        // println!("{present:?}");
        presents.push(present);
        for s in 0..8 {
            presents[i].shapes.push(shape.clone());
            rotate(&mut shape);
            if s == 3 {
                flip(&mut shape);
            }
        }
    }

    let mut ans = 0;
    for line in lines[5 * 6..lines.len()].iter() {
        let (size_str, counts_str) = line.split_once(": ").unwrap();
        let (width, height) = size_str
            .split_once('x')
            .map(|(ws, hs)| (ws.parse().unwrap(), hs.parse().unwrap()))
            .unwrap();
        let counts: Vec<usize> = counts_str.split(' ').map(|s| s.parse().unwrap()).collect();

        let mut grid: Vec<Vec<usize>> = vec![vec![0; width]; height];

        // let found = brute_force(&mut grid, &presents, &counts, 0, 0);
        // println!("{found}");
        // for row in grid {
        //     println!("{row:?}");
        // }
        // if found {
        //     ans += 1;
        // }

        let mut sum = 0;
        for i in 0..6 {
            sum += counts[i] * presents[i].tile_count;
        }
        let res = sum <= width*height;
        if res {
            ans += 1;
        }
        // println!("{res} -- {sum} {}", width*height);
    }

    println!("{ans}");
}

fn rotate(shape: &mut Vec<Vec<bool>>) {
    let n = shape.len();
    assert!(shape.len() == shape[0].len());

    let shape_copy = shape.clone();

    for i in 0..n {
        for j in 0..n {
            shape[i][j] = shape_copy[n - j - 1][i];
        }
    }
}

fn flip(shape: &mut Vec<Vec<bool>>) {
    let n = shape.len();
    assert!(shape.len() == shape[0].len());

    for i in 0..n / 2 {
        shape.swap(i, n - i - 1);
    }
}

fn brute_force(
    grid: &mut Vec<Vec<usize>>,
    presents: &Vec<Present>,
    counts: &Vec<usize>,
    i: usize,
    cnt: usize,
) -> bool {
    let n = presents.len();
    if i == n {
        return true;
    }
    let m = counts[i];
    if cnt == m {
        return brute_force(grid, presents, counts, i + 1, 0);
    }

    for r in 0..grid.len() - 2 {
        for c in 0..grid[0].len() - 2 {
            for s in 0..8 {
                let shape = &presents[i].shapes[s];
                // println!("{i} {cnt} {s}");
                // for row in shape {
                //     println!("{row:?}");
                // }
                let mut fits = true;
                for rr in 0..3 {
                    for cc in 0..3 {
                        if shape[rr][cc] {
                            grid[r + rr][c + cc] += 1;
                        }
                        if grid[r + rr][c + cc] > 1 {
                            fits = false;
                        }
                    }
                }
                // for row in grid.iter() {
                //     println!("{row:?}");
                // }
                // thread::sleep(Duration::from_millis(500));
                if fits && brute_force(grid, presents, counts, i, cnt + 1) {
                    return true;
                }
                for rr in 0..3 {
                    for cc in 0..3 {
                        if shape[rr][cc] {
                            grid[r + rr][c + cc] -= 1;
                        }
                    }
                }
            }
        }
    }

    false
}
