fn split_pairs(pair: &str) -> ((u32, u32), (u32, u32)) {
    let assignments: Vec<u32> = pair
        .split(",")
        .flat_map(|a| a.split("-").map(|s| s.parse::<u32>().unwrap()))
        .collect();
    (
        (assignments[0], assignments[1]),
        (assignments[2], assignments[3]),
    )
}

fn part_one(pair: &str) -> u32 {
    let ((b1, e1), (b2, e2)) = split_pairs(pair);
    if (b2 <= b1 && e2 >= e1) || (b1 <= b2 && e1 >= e2) {
        1
    } else {
        0
    }
}

fn part_two(pair: &str) -> u32 {
    let ((b1, e1), (b2, e2)) = split_pairs(pair);
    if b2 >= b1 && b2 <= e1 {
        1
    } else if b1 >= b2 && b1 <= e2 {
        1
    } else {
        0
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day04.txt").unwrap();
    let part_one: u32 = input
        .split("\n")
        .filter(|r| r.len() > 0)
        .map(part_one)
        .sum();
    println!("Part one: {part_one}");

    let part_two: u32 = input
        .split("\n")
        .filter(|r| r.len() > 0)
        .map(part_two)
        .sum();
    println!("Part two: {part_two}");
}
