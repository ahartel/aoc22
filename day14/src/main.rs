use itertools::Itertools;

fn load_initial_scan<'a>(rows: impl Iterator<Item = &'a str>) -> (usize, Vec<Vec<u8>>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = usize::MAX;
    let paths: Vec<Vec<(usize, usize)>> = rows
        .map(|row| {
            row.split(" -> ")
                .map(|coordinate| to_tuple(coordinate, &mut max_x, &mut min_x, &mut max_y))
                .collect_vec()
        })
        .collect_vec();
    println!("{min_x}..{max_x}");
    let mut scan: Vec<Vec<u8>> = vec![vec![0; max_x - min_x + 1]; max_y + 1];
    for path in paths {
        for ((x1, y1), (x2, y2)) in path.into_iter().tuple_windows() {
            if x1 == x2 {
                if y1 < y2 {
                    for i in y1..y2 + 1 {
                        scan[i][x1 - min_x] = 1;
                    }
                } else {
                    for i in y2..y1 + 1 {
                        scan[i][x1 - min_x] = 1;
                    }
                }
            } else {
                if x1 < x2 {
                    for i in x1..x2 + 1 {
                        scan[y1][i - min_x] = 1;
                    }
                } else {
                    for i in x2..x1 + 1 {
                        scan[y1][i - min_x] = 1;
                    }
                }
            }
        }
    }
    (500 - min_x, scan)
}

fn to_tuple(
    coordinate: &str,
    max_x: &mut usize,
    min_x: &mut usize,
    max_y: &mut usize,
) -> (usize, usize) {
    let (x, y) = coordinate
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .tuples()
        .next()
        .unwrap();
    if x < *min_x {
        *min_x = x;
    }
    if x > *max_x {
        *max_x = x;
    }
    if y > *max_y {
        *max_y = y;
    }
    (x, y)
}

fn print_scan(scan: &[Vec<u8>]) {
    for row in scan {
        for val in row {
            print!("{val}");
        }
        println!("");
    }
}

fn simulate_sand(scan: &mut Vec<Vec<u8>>, init_x: usize) -> usize {
    let mut rest = true;
    let mut rested = 0;
    while rest {
        let mut pos = (init_x, 0);
        rest = false;
        for _ in 0..250 {
            let (x, y) = pos;
            if y == scan.len() - 1 || x == scan[0].len() || x == 0 {
                break;
            }
            if scan[y + 1][x] == 0 {
                pos = (x, y + 1);
            } else if x > 0 && scan[y + 1][x - 1] == 0 {
                pos = (x - 1, y + 1);
            } else if x < scan[0].len() - 1 && scan[y + 1][x + 1] == 0 {
                pos = (x + 1, y + 1);
            } else {
                rest = true;
                scan[y][x] = 2;
            }
        }
        if rest {
            rested += 1;
        }
    }
    rested
}

fn main() {
    let input = std::fs::read_to_string("../input/day14.txt").unwrap();
    let (init_x, mut scan) = load_initial_scan(input.split("\n"));
    print_scan(&scan);
    println!("{}, {}", scan[0].len(), scan.len());
    let rested = simulate_sand(&mut scan, init_x);
    print_scan(&scan);
    println!("Part one: rested {rested}");
}

#[cfg(test)]
mod tests {
    use crate::{load_initial_scan, simulate_sand};

    #[test]
    fn example_case_part_one() {
        let rows = [
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];
        let (init_x, mut scan) = load_initial_scan(rows.into_iter());
        println!("{}, {}", scan[0].len(), scan.len());
        let rested = simulate_sand(&mut scan, init_x);
        assert_eq!(rested, 24);
    }
}
