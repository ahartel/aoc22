/// --- Day 8: Treetop Tree House ---
///
/// The expedition comes across a peculiar patch of tall trees all planted
/// carefully in a grid. The Elves explain that a previous expedition planted
/// these trees as a reforestation effort. Now, they're curious if this would be
/// a good location for a tree house.
///
/// First, determine whether there is enough tree cover here to keep a tree
/// house hidden. To do this, you need to count the number of trees that are
/// visible from outside the grid when looking directly along a row or column.
///
/// The Elves have already launched a quadcopter to generate a map with the
/// height of each tree (your puzzle input). For example:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
///
/// Each tree is represented as a single digit whose value is its height, where
/// 0 is the shortest and 9 is the tallest.
///
/// A tree is visible if all of the other trees between it and an edge of the
/// grid are shorter than it. Only consider trees in the same row or column;
/// that is, only look up, down, left, or right from any given tree.
///
/// All of the trees around the edge of the grid are visible - since they are
/// already on the edge, there are no trees to block the view. In this example,
/// that only leaves the interior nine trees to consider:
///
///   - The top-left 5 is visible from the left and top. (It isn't visible from
///     the right or bottom since other trees of height 5 are in the way.)
///   - The top-middle 5 is visible from the top and right.
///   - The top-right 1 is not visible from any direction; for it to be visible,
///     there would need to only be trees of height 0 between it and an edge.
///   - The left-middle 5 is visible, but only from the right.
///   - The center 3 is not visible from any direction; for it to be visible,
///     there would need to be only trees of at most height 2 between it and an
///     edge.
///   - The right-middle 3 is visible from the right.
///     In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
///
/// With 16 trees visible on the edge and another 5 visible in the interior, a
/// total of 21 trees are visible in this arrangement.
///
/// Consider your map; how many trees are visible from outside the grid?
///
/// --- Part Two ---
///
/// Content with the amount of tree cover available, the Elves just need to know the best
/// spot to build their tree house: they would like to be able to see a lot of trees.
///
/// To measure the viewing distance from a given tree, look up, down, left, and right from
/// that tree; stop if you reach an edge or at the first tree that is the same height or
/// taller than the tree under consideration. (If a tree is right on the edge, at least
/// one of its viewing distances will be zero.)
///
/// The Elves don't care about distant trees taller than those found by the rules above;
/// the proposed tree house has large eaves to keep it dry, so they wouldn't be able to
/// see higher than the tree house anyway.
///
/// In the example above, consider the middle 5 in the second row:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
///
///   - Looking up, its view is not blocked; it can see 1 tree (of height 3).
///   - Looking left, its view is blocked immediately; it can see only 1 tree (of height
///     5, right next to it).
///   - Looking right, its view is not blocked; it can see 2 trees.
///   - Looking down, its view is blocked eventually; it can see 2 trees (one of height
///     3, then the tree of height 5 that blocks its view).
///
/// A tree's scenic score is found by multiplying together its viewing distance in each of
/// the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
///
/// However, you can do even better: consider the tree of height 5 in the middle of the
/// fourth row:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
///
///   - Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
///   - Looking left, its view is not blocked; it can see 2 trees.
///   - Looking down, its view is also not blocked; it can see 1 tree.
///   - Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
///
/// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree
/// house.
///
/// Consider each tree on your map. What is the highest scenic score possible for any
/// tree?

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<u8>> {
    lines
        .map(|line| {
            println!("{line}");
            line.chars().map(|c| (c as u8) - 48).collect::<Vec<u8>>()
        })
        .collect()
}

fn print_visible(visible: &[Vec<bool>]) {
    println!("");
    for row in visible {
        for col in row {
            if *col {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn print_score(visible: &[Vec<u32>]) {
    println!("");
    for row in visible {
        for col in row {
            print!("{col:0>2}|");
        }
        print!("\n");
    }
}

fn find_visible<'a>(trees: &[Vec<u8>]) -> Vec<Vec<bool>> {
    let visible: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| {
                    if row_idx == 0 || col_idx == 0 {
                        true
                    } else if row_idx == trees.len() - 1 || col_idx == row.len() - 1 {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let visible = find_visible_from_left(&trees, &visible);
    print_visible(&visible);
    let visible = find_visible_from_right(&trees, &visible);
    print_visible(&visible);
    let visible = find_visible_from_top(&trees, &visible);
    print_visible(&visible);
    let visible = find_visible_from_bottom(&trees, &visible);
    print_visible(&visible);
    visible
}

fn find_visible_from_top(trees: &[Vec<u8>], visible: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut max_height_in_cols: Vec<u8> = vec![0; trees.len()];
    trees
        .iter()
        .zip(visible)
        .map(|(row, vis)| {
            row.iter()
                .zip(vis)
                .enumerate()
                .map(|(col_idx, (&t, v))| {
                    if t > max_height_in_cols[col_idx] {
                        max_height_in_cols[col_idx] = t;
                        true
                    } else {
                        *v
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_visible_from_bottom(trees: &[Vec<u8>], visible: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut max_height_in_cols: Vec<u8> = vec![0; trees.len()];
    let mut new_visible: Vec<Vec<bool>> = trees
        .iter()
        .zip(visible)
        .rev()
        .map(|(row, vis)| {
            row.iter()
                .zip(vis)
                .enumerate()
                .map(|(col_idx, (&t, v))| {
                    if t > max_height_in_cols[col_idx] {
                        max_height_in_cols[col_idx] = t;
                        true
                    } else {
                        *v
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    new_visible.reverse();
    new_visible
}

fn main() {
    let input = std::fs::read_to_string("../input/day08.txt").unwrap();
    let trees = parse_input(input.split("\n"));
    let visible = find_visible(&trees);
    let num = visible.iter().flat_map(|row| row).filter(|&v| *v).count();
    println!("Part one: {num}");

    let score = calculate_scenic_score(&trees);
    let max = score.iter().flat_map(|row| row).max().unwrap();
    println!("Part two: {max:?}");
}

fn find_visible_from_left(trees: &[Vec<u8>], visible: &[Vec<bool>]) -> Vec<Vec<bool>> {
    trees
        .iter()
        .zip(visible)
        .map(|(row, vis)| {
            let mut max_height_in_row: u8 = 0;
            row.iter()
                .zip(vis)
                .map(|(&t, v)| {
                    if t > max_height_in_row {
                        max_height_in_row = t;
                        true
                    } else {
                        *v
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_visible_from_right(trees: &[Vec<u8>], visible: &[Vec<bool>]) -> Vec<Vec<bool>> {
    trees
        .iter()
        .zip(visible)
        .map(|(row, vis)| {
            let mut max_height_in_row: u8 = 0;
            let mut new_row = row
                .iter()
                .zip(vis)
                .rev()
                .map(|(&t, v)| {
                    if t > max_height_in_row {
                        max_height_in_row = t;
                        true
                    } else {
                        *v
                    }
                })
                .collect::<Vec<_>>();
            new_row.reverse();
            new_row
        })
        .collect()
}

fn calculate_scenic_score<'a>(trees: &[Vec<u8>]) -> Vec<Vec<u32>> {
    let score: Vec<Vec<u32>> = trees
        .iter()
        .map(|row| row.iter().map(|_| 1).collect::<Vec<_>>())
        .collect();
    let score = calc_score_from_left(&trees, &score);
    let score = calc_score_from_right(&trees, &score);
    let score = calc_score_from_top(&trees, &score);
    let score = calc_score_from_bottom(&trees, &score);
    score
}

fn calc_score_from_left(trees: &[Vec<u8>], scores: &[Vec<u32>]) -> Vec<Vec<u32>> {
    trees
        .iter()
        .zip(scores)
        .map(|(row, row_scores)| {
            let mut max_pos_of_height: [usize; 10] = [0; 10];
            row.iter()
                .zip(row_scores)
                .enumerate()
                .map(|(col_idx, (&t, &s))| {
                    let old_idx = max_pos_of_height[t as usize];
                    if col_idx > old_idx {
                        for i in 0..(t + 1) {
                            max_pos_of_height[i as usize] = col_idx;
                        }
                    }
                    s * (col_idx - old_idx) as u32
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn calc_score_from_right(trees: &[Vec<u8>], scores: &[Vec<u32>]) -> Vec<Vec<u32>> {
    trees
        .iter()
        .zip(scores)
        .map(|(row, row_scores)| {
            let mut max_pos_of_height: [usize; 10] = [row.len() - 1; 10];
            let mut new_scores = row
                .iter()
                .zip(row_scores)
                .enumerate()
                .rev()
                .map(|(col_idx, (&t, &s))| {
                    let old_idx = max_pos_of_height[t as usize];
                    if col_idx < old_idx {
                        for i in 0..(t + 1) {
                            max_pos_of_height[i as usize] = col_idx;
                        }
                    }
                    s * (old_idx - col_idx) as u32
                })
                .collect::<Vec<_>>();
            new_scores.reverse();
            new_scores
        })
        .collect()
}

fn calc_score_from_top(trees: &[Vec<u8>], scores: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut max_pos_of_height: Vec<[usize; 10]> = vec![[0; 10]; trees[0].len()];
    trees
        .iter()
        .zip(scores)
        .enumerate()
        .map(|(row_idx, (row, row_scores))| {
            row.iter()
                .zip(row_scores)
                .enumerate()
                .map(|(col_idx, (&t, &s))| {
                    let old_idx = max_pos_of_height[col_idx][t as usize];
                    if row_idx > old_idx {
                        for i in 0..(t + 1) {
                            max_pos_of_height[col_idx][i as usize] = row_idx;
                        }
                    }
                    s * (row_idx - old_idx) as u32
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn calc_score_from_bottom(trees: &[Vec<u8>], scores: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut max_pos_of_height: Vec<[usize; 10]> = vec![[trees.len() - 1; 10]; trees[0].len()];
    let mut new_scores: Vec<Vec<u32>> = trees
        .iter()
        .zip(scores)
        .enumerate()
        .rev()
        .map(|(row_idx, (row, row_scores))| {
            row.iter()
                .zip(row_scores)
                .enumerate()
                .map(|(col_idx, (&t, &s))| {
                    let old_idx = max_pos_of_height[col_idx][t as usize];
                    if row_idx < old_idx {
                        for i in 0..(t + 1) {
                            max_pos_of_height[col_idx][i as usize] = row_idx;
                        }
                    }
                    s * (old_idx - row_idx) as u32
                })
                .collect::<Vec<_>>()
        })
        .collect();
    new_scores.reverse();
    new_scores
}

#[cfg(test)]
mod tests {
    use crate::{calculate_scenic_score, find_visible, parse_input, print_score};

    #[test]
    fn example_case_part_one() {
        let raw_data = vec!["30373", "25512", "65332", "33549", "35390"];
        let visible = find_visible(&parse_input(raw_data.into_iter()));
        let num = visible.iter().flat_map(|row| row).filter(|&v| *v).count();
        assert_eq!(num, 21);
    }

    #[test]
    fn example_case_part_two() {
        let raw_data = vec!["30373", "25512", "65332", "33549", "35390"];
        let score = calculate_scenic_score(&parse_input(raw_data.into_iter()));
        print_score(&score);
        let max = score.into_iter().flat_map(|row| row).max().unwrap();
        println!("Max: {max}");
        assert_eq!(max, 8);
    }
}
