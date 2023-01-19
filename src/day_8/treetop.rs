use crate::utils::file::{get_lines, unwrap_line_results_v2};

pub fn part1(file_name: &String) {
    let line_results = get_lines(file_name);
    let lines = unwrap_line_results_v2(&line_results);

    let mut cols: Vec<Vec<u8>> = Vec::new();
    let mut rows: Vec<Vec<u8>> = Vec::new();

    for (line_number, line) in lines.iter().enumerate() {
        let trees = parse_line(line);
        if line_number == 0 {
            cols = initiate_cols(&trees);
        } else {
            for (tree_number, tree) in trees.iter().enumerate() {
                cols[tree_number].push(*tree);
            }
        }
        rows.push(trees);
    }

    let n = rows.len() - 1;

    let mut count_of_visible_trees = 0;

    let mut tree_scores: Vec<u32> = Vec::new();

    for row_idx in 1..n {
        for col_idx in 1..n {
            let current_row = &rows[row_idx];
            let current_col = &cols[col_idx];

            // current tree
            let current_tree = current_row[col_idx];

            // check column
            // highest tree above current tree
            let trees_above = &current_col[0..row_idx];
            // highest tree below current tree
            let trees_below = &current_col[row_idx + 1..n + 1];
            // check rows
            // highest tree left of current tree
            let trees_left = &current_row[0..col_idx];

            // highest tree right of current tree
            let trees_right = &current_row[col_idx + 1..n + 1];

            let trees_around = vec![trees_above, trees_below, trees_left, trees_right];

            let tree_score: u32 = trees_around
                .iter()
                .enumerate()
                .fold(1, |acc, (idx, trees)| {
                    acc * find_distance(*trees, current_tree, idx % 2 == 0)
                });

            tree_scores.push(tree_score);

            let highest_trees = trees_around
                .iter()
                .map(|i| i.iter().max().unwrap_or(&0))
                .collect::<Vec<&u8>>();

            if &current_tree > highest_trees.iter().min().unwrap() {
                count_of_visible_trees += 1;
            }
        }
    }
    println!("inner visible trees {:?}", count_of_visible_trees);
    println!("outer visible trees {:?}", n * 4);
    println!("all visible trees {:?}", n * 4 + count_of_visible_trees);

    println!("maximum score {:?}", tree_scores.iter().max().unwrap());
}

pub fn find_distance(trees: &[u8], tree_height: u8, reverse: bool) -> u32 {
    if reverse {
        for (index, tree) in trees.iter().rev().enumerate() {
            if tree >= &&tree_height {
                return (index + 1).try_into().unwrap();
            }
        }
    } else {
        for (index, tree) in trees.iter().enumerate() {
            if tree >= &&tree_height {
                return (index + 1).try_into().unwrap();
            }
        }
    }
    (trees.len()).try_into().unwrap()
}

pub fn parse_line(line: &String) -> Vec<u8> {
    line.split("")
        .filter(|j| **j != "".to_string())
        .map(|i| i.parse::<u8>().unwrap())
        .collect()
}

pub fn initiate_cols(trees: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut t: Vec<Vec<u8>> = Vec::new();
    for tree in trees {
        t.push(vec![*tree]);
    }
    t
}
