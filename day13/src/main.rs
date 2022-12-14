fn compare_lists<'a>(mut input: impl Iterator<Item = &'a str>) {
    while let Some(first_row) = input.next() {
        if let Some(second_row) = input.next() {
            println!("{first_row}");
            println!("{second_row}");
            input.next();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day13.txt").unwrap();
    let input = input.split("\n");
    compare_lists(input);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::compare_lists;

    #[test]
    fn example_case() {
        let data = vec![
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ];
        compare_lists(data.into_iter());
    }
}
