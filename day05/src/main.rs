fn prepare_data<'a>(
    mut rows: impl Iterator<Item = &'a str>,
) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    // collect the first rows, those containing the starting crate state
    let starting_positions: Vec<&str> = (&mut rows)
        .take_while(|&row| !row.starts_with(" 1"))
        .collect();
    // construct the crate state
    let mut num_empty = 0;
    let mut crate_state: Vec<Vec<char>> = vec![vec![]; 9];
    for row in starting_positions.into_iter().rev() {
        let mut col_idx = 0;
        for col in row.split(" ") {
            if col == "" {
                num_empty += 1;
                if num_empty == 4 {
                    col_idx += 1;
                    num_empty = 0;
                }
            } else {
                crate_state[col_idx].push(col.chars().skip(1).next().unwrap());
                col_idx += 1;
            }
        }
    }
    // transform the instructions
    let instructions = rows
        .skip(1)
        .map(|row| {
            let split: Vec<&str> = row.split(" ").collect();
            let num = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap() - 1;
            let to = split[5].parse::<usize>().unwrap() - 1;
            (num, from, to)
        })
        .collect();

    (crate_state, instructions)
}

fn solve_part_one(
    mut crate_state: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
) -> String {
    instructions.into_iter().for_each(|(num, from, to)| {
        for _ in 0..num {
            let taken = crate_state[from].pop().unwrap();
            crate_state[to].push(taken);
        }
    });
    crate_state.iter().map(|col| col.last().unwrap()).collect()
}

fn solve_part_two(
    mut crate_state: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
) -> String {
    instructions.into_iter().for_each(|(num, from, to)| {
        let length = crate_state[from].len();
        let taken: Vec<char> = crate_state[from].drain(length - num..length).collect();
        for take in taken.into_iter() {
            crate_state[to].push(take);
        }
    });
    crate_state.iter().map(|col| col.last().unwrap()).collect()
}

fn main() {
    let input = std::fs::read_to_string("../input/day05.txt").unwrap();
    let (crate_state, instructions) = prepare_data(input.split("\n"));

    println!(
        "Part one: {}",
        solve_part_one(crate_state.clone(), instructions.clone())
    );

    println!("Part two: {}", solve_part_two(crate_state, instructions));
}
