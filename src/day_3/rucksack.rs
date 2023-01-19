use std::collections::HashSet;

use crate::utils::{
    alphabet::{lower_case_alphabet, upper_case_alphabet},
    file::*,
};

pub fn part1(file_name: &String) {
    let low = lower_case_alphabet();
    let up = upper_case_alphabet();
    let lines = get_lines(file_name);
    let mut score = 0;
    for line_result in lines {
        let line = line_result.unwrap();

        let (comp_one, comp_two) = line.split_at(line.len() / 2);
        println!("{:?}", comp_one);
        println!("{:?}", comp_two);

        let set_comp_one: HashSet<char> = comp_one.chars().collect();

        let matches: Vec<char> = comp_two
            .chars()
            .filter(|c| set_comp_one.contains(&c))
            .collect();

        let common_item: char = matches[0];

        let local_score: usize;

        if common_item.is_uppercase() {
            println!("{:?} --> {:?}", common_item, up.find(common_item));
            local_score = up.find(common_item).unwrap_or_else(|| 0) + 26;
        } else {
            println!("{:?}", low.find(common_item));
            local_score = low.find(common_item).unwrap_or_else(|| 0);
        }

        println!("full line: {:?}", line);
        println!("match: {:?}", matches);
        println!("local score: {:?}", local_score);
        score += local_score + 1;
        println!("score: {:?}", score);

        print!("\n");
    }
    println!("score: {:?}", score);

    println!(
        "this is a test: {:?}",
        "my_test_string".to_string().find("m")
    );
}

pub fn part2(file_name: &String) {
    println!("Day 3 - part 2\n");
    let low = lower_case_alphabet();
    let up = upper_case_alphabet();
    let line_results = get_lines(file_name);
    let line_contents = unwrap_line_results(&line_results);
    let chunks = line_contents.chunks(3);
    //let chunks = split_lines_by_count(3, &line_results);
    let mut score = 0;
    for chunk in chunks.into_iter() {
        println!("{:?}", chunk);
        let first_item = &chunk[0];
        let second_item = &chunk[1];
        let third_item = &chunk[2];

        let first_item_hash: HashSet<char> = first_item.chars().collect();
        let second_item_hash: HashSet<char> = second_item.chars().collect();

        let badge: char = third_item
            .chars()
            .filter(|c| first_item_hash.contains(c) && second_item_hash.contains(c))
            .nth(0)
            .unwrap();

        let local_score: usize;

        println!("{:?}\n", badge);

        if badge.is_uppercase() {
            println!("{:?} --> {:?}", badge, up.find(badge));
            local_score = up.find(badge).unwrap_or_else(|| 0) + 26;
        } else {
            println!("{:?}", low.find(badge));
            local_score = low.find(badge).unwrap_or_else(|| 0);
        }

        score += local_score + 1;

        println!("local score: {:?}", local_score);
    }
    println!("total score {:?}", score);
}
