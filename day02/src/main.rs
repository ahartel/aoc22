fn evaulate_round_one(round: &str) -> u32 {
    match chars_to_tuple_one(round) {
        (1, 2) => 6 + 2,
        (2, 3) => 6 + 3,
        (3, 1) => 6 + 1,
        (1, 1) => 3 + 1,
        (2, 2) => 3 + 2,
        (3, 3) => 3 + 3,
        (_, b) => b,
    }
}

fn evaulate_round_two(round: &str) -> u32 {
    match chars_to_tuple_two(round) {
        (a, 3) => a + 3,
        (1, 0) => 3,
        (2, 0) => 1,
        (3, 0) => 2,
        (1, 6) => 2 + 6,
        (2, 6) => 3 + 6,
        (3, 6) => 1 + 6,
        _ => 0,
    }
}

fn chars_to_tuple_one(round: &str) -> (u32, u32) {
    match round {
        "A X" => (1, 1),
        "A Y" => (1, 2),
        "A Z" => (1, 3),
        "B X" => (2, 1),
        "B Y" => (2, 2),
        "B Z" => (2, 3),
        "C X" => (3, 1),
        "C Y" => (3, 2),
        "C Z" => (3, 3),
        _ => (0, 0),
    }
}

fn chars_to_tuple_two(round: &str) -> (u32, u32) {
    match round {
        "A X" => (1, 0),
        "A Y" => (1, 3),
        "A Z" => (1, 6),
        "B X" => (2, 0),
        "B Y" => (2, 3),
        "B Z" => (2, 6),
        "C X" => (3, 0),
        "C Y" => (3, 3),
        "C Z" => (3, 6),
        _ => (0, 0),
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day02.txt").unwrap();
    let part_one: u32 = input.split("\n").map(evaulate_round_one).sum();
    println!("Part one: {part_one}");
    let part_two: u32 = input.split("\n").map(evaulate_round_two).sum();
    println!("Part two: {part_two}");
}
