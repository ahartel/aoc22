use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("../input/day01.txt").unwrap();
    let elves = input
        .split("\n")
        .scan(vec![], |state: &mut Vec<u32>, row| {
            if row == "" {
                let ret = state.clone();
                *state = vec![];
                Some(Some(ret))
            } else {
                state.push(str::parse::<u32>(row).unwrap());
                Some(None)
            }
        })
        .filter_map(|elf| elf)
        .map(|elf| elf.iter().sum::<u32>())
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    println!("Part One: {:?}", elves.iter().max().unwrap());
    println!("Part Two: {:?}", elves.iter().take(3).sum::<u32>());
}
