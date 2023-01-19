use crate::{
    day_7::log::{is_command, is_move_command, is_move_up_command},
    utils::file::{get_lines, unwrap_line_results_v2},
};
use std::collections::HashMap;

pub fn part1(file_name: &String) {
    let mut path_list: HashMap<String, (u32, Vec<String>)> = HashMap::new();

    let logs = get_logs(file_name);

    let mut positions: Vec<String> = Vec::new();

    for (i, line) in logs.into_iter().enumerate() {
        if is_command(&line) {
            if is_move_command(&line) {
                if is_move_up_command(&line) {
                    let l = positions.len() - 1;
                    positions.truncate(l);
                } else {
                    positions.push(line[2].to_string());
                    let current_full_path = positions.to_vec().join("-");
                    path_list.insert(current_full_path, (0, vec![]));
                }
            }
        } else {
            let current_position = positions.clone().join("-");

            let (current_file_size, current_children_dir) = path_list
                .entry(current_position.to_string())
                .or_insert((0, vec![]));

            if line[0] == "dir" {
                let new_dir = &line[1];

                let copy_of_current_position = current_position + "-" + new_dir;

                current_children_dir.push(copy_of_current_position);
            } else {
                let new_file_size = line[0].parse::<u32>().unwrap();
                *current_file_size += new_file_size;
            }
        }
    }

    let mut all_sizes: Vec<u32> = Vec::new();

    for (mut x, mut y) in path_list.clone().into_iter() {
        let (mut file_size, _) = y;
        let mut full_file_size = get_file_size_of_descendants(&path_list, &y);
        all_sizes.push(full_file_size);
    }

    let part_1_answer: u32 = all_sizes.iter().filter(|i| **i < 100000).sum();

    println!(
        "part 1 answer: {:?}\n--------------------------",
        part_1_answer
    );

    let mut root_size = all_sizes.iter().max().unwrap();
    let mut size_of_update = 30000000;
    let mut total_space_of_device = 70000000;

    let mut total_free_space = total_space_of_device - root_size;
    let mut total_missing_space = size_of_update - total_free_space;

    println!("total occupied space {:?}", root_size,);
    println!("total free space {:?}", total_free_space);
    println!("total missing space {:?}", total_missing_space);

    let part2 = all_sizes
        .iter()
        .filter(|i| **i >= total_missing_space)
        .min()
        .unwrap();

    println!("part 2  answer: {:?}", part2);
}

fn get_file_size_of_descendants<'a>(
    path_list: &'a HashMap<String, (u32, Vec<String>)>,
    contents: &'a (u32, Vec<String>),
) -> u32 {
    let final_size = 0;
    let (mut size, child_paths) = contents;
    if (!child_paths.is_empty()) {
        for child_path in child_paths {
            let child_dir_contents = path_list.get(child_path).unwrap();
            size += get_file_size_of_descendants(path_list, child_dir_contents);
        }
    }
    final_size + size
}

fn get_logs(file_name: &String) -> Vec<Vec<String>> {
    let line_results = get_lines(file_name);
    let lines = unwrap_line_results_v2(&line_results);

    let terminal_logs = lines
        .into_iter()
        .map(clean_log)
        .collect::<Vec<Vec<String>>>();

    terminal_logs
}

fn clean_log(line: String) -> Vec<String> {
    let res = line
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    res
}
