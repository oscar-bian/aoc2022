use std::collections::HashMap;

use crate::utils::file::{get_lines, unwrap_line_results};

pub fn part2(file_name: &String) {
    let (mut crates, move_list) = parse_file(file_name);

    for next_move in move_list {
        let n = next_move[0];

        let origin_pile_index = next_move[1] - 1;
        let destination_pile_index = next_move[2] - 1;

        let origin_pile = crates[origin_pile_index].clone();
        let mut destination_pile = crates[destination_pile_index].clone();

        let (crates_to_leave, crates_to_move) = origin_pile.split_at(origin_pile.len() - n);

        crates[origin_pile_index] = crates_to_leave.to_vec();
        destination_pile.append(&mut crates_to_move.to_vec());
        crates[destination_pile_index] = destination_pile;
    }

    print_results(&crates);
}

pub fn part1(file_name: &String) {
    let (mut crates, move_list) = parse_file(file_name);

    for next_move in move_list {
        let n = next_move[0];

        let origin_pile_index = next_move[1] - 1;
        let destination_pile_index = next_move[2] - 1;

        let mut origin_pile = crates[origin_pile_index].clone();
        let mut destination_pile = crates[destination_pile_index].clone();

        for _ in 0..n {
            let crate_from_origin = origin_pile.pop().unwrap();
            destination_pile.push(crate_from_origin);
        }

        crates[origin_pile_index] = origin_pile.to_vec();
        crates[destination_pile_index] = destination_pile.to_vec();
    }

    print_results(&crates);
}

fn print_results(crates: &Vec<Vec<String>>) {
    let mut result = "".to_owned();

    for item in crates {
        result.push_str(&item.last().unwrap());
    }

    println!("{:?}", result);
}

fn parse_file(file_name: &String) -> (Vec<Vec<String>>, Vec<Vec<usize>>) {
    let line_results = get_lines(file_name);
    let lines = unwrap_line_results(&line_results);

    let separator = 9;

    let crates = parse_crates_v2(&lines[0..separator - 1]);

    let move_list = parse_moves(&lines[separator + 1..]);

    (crates, move_list)
}

fn parse_moves(lines: &[&String]) -> Vec<Vec<usize>> {
    let move_list: Vec<Vec<usize>>;

    move_list = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|c| c.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    move_list
}

fn parse_crates_v2(lines: &[&String]) -> Vec<Vec<String>> {
    let mut crates: Vec<Vec<String>> = Vec::new();
    for line in lines.iter().rev() {
        let crate_rows = line
            .chars()
            .skip(1)
            .step_by(4)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        for (row_index, crate_cell) in crate_rows.iter().enumerate() {
            if crate_cell != " " {
                if crates.len() > row_index {
                    let crate_stack = &mut crates[row_index];
                    crate_stack.push(crate_cell.to_string());
                    crates[row_index] = crate_stack.to_vec();
                } else {
                    crates.push(vec![crate_cell.to_string()]);
                }
            }
        }
    }

    crates
}

fn parse_crates(lines: &[&String]) -> HashMap<usize, Vec<String>> {
    let mut crates: HashMap<usize, Vec<String>> = HashMap::new();

    for line in lines.iter().rev() {
        let crate_rows = line
            .chars()
            .skip(1)
            .step_by(4)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        for (row_index, crate_cell) in crate_rows.iter().enumerate() {
            if crate_cell != " " {
                let crate_number = row_index + 1;
                if crates.contains_key(&crate_number) {
                    let mut old = crates.get(&crate_number).unwrap().clone();
                    old.push(crate_cell.to_string());
                    crates.insert(crate_number, old.to_vec());
                } else {
                    crates.insert(crate_number, vec![crate_cell.to_string()]);
                }
            }
        }
    }
    crates
}
