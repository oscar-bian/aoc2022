use std::collections::HashSet;

use crate::utils::file::*;

pub fn part1(file_name: &String) {
    let line_results = get_lines(file_name);
    let contents = unwrap_line_results_v2(&line_results);

    find_n_char_repetition(&contents[0], 4);
}

pub fn part2(file_name: &String) {
    let line_results = get_lines(file_name);
    let contents = unwrap_line_results_v2(&line_results);

    find_n_char_repetition(&contents[0], 14);
}

fn find_n_char_repetition(contents: &String, n: usize) {
    let mut found_first = false;

    for (i, _) in contents.chars().enumerate() {
        if i < contents.len() - (n - 1) {
            let n_chars = &contents[i..i + n];

            let n_chars_hash: HashSet<char> = n_chars.chars().into_iter().collect();

            if n_chars_hash.len() == n && !found_first {
                println!("{:?} --> {:?} --> {:?}\n", i + n, n_chars, n_chars_hash);
                found_first = true;
            }
        }
    }
}
