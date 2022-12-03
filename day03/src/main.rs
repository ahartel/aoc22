use std::collections::HashSet;
use std::mem;

fn ctoi(c: char) -> usize {
    let ascii_code: u32 = c.into();
    if ascii_code > 96 {
        // small letters
        ascii_code as usize - 96
    } else {
        // capital letters
        ascii_code as usize - 38
    }
}

fn evaulate_round_one(row: &str) -> usize {
    let mut found = HashSet::new();
    let mid = row.len() / 2;
    for c in row[0..mid].chars() {
        found.insert(c);
    }
    for c in row[mid..].chars() {
        if found.contains(&c) {
            return ctoi(c);
        }
    }
    0
}

fn main() {
    let input = std::fs::read_to_string("../input/day03.txt").unwrap();

    let part_one: usize = input.split("\n").map(evaulate_round_one).sum();
    println!("Part one: {part_one}");

    let part_two: u32 = input
        .split("\n")
        .map(|row| row.chars().collect::<HashSet<char>>())
        .scan(HashSet::<char>::new(), |state, row| {
            if state.len() == 0 {
                *state = row;
            } else {
                *state = state.intersection(&row).cloned().collect();
            }
            if state.len() == 1 {
                Some(Some(mem::replace(state, HashSet::new())))
            } else {
                Some(None)
            }
        })
        .filter_map(|g| g)
        .map(|g| ctoi(*g.iter().next().unwrap()) as u32)
        .sum();
    println!("Part two: {part_two}");
}
