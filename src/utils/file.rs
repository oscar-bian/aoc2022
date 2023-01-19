use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub fn get_lines(file_name: &String) -> Vec<Result<String, Error>> {
    let lines = BufRead::lines(BufReader::new(File::open(file_name).unwrap())).collect::<Vec<_>>();
    lines
}

pub fn get_lines_v2(file_name: &String) -> Vec<String> {
    BufRead::lines(BufReader::new(File::open(file_name).unwrap()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|line| line.unwrap())
        .collect()
}

pub fn unwrap_line_results(line_results: &Vec<Result<String, Error>>) -> Vec<&String> {
    let lines_unwrapped = line_results
        .into_iter()
        .map(|line| line.as_ref().unwrap())
        .collect::<Vec<&String>>();

    lines_unwrapped
}

pub fn unwrap_line_results_v2(line_results: &Vec<Result<String, Error>>) -> Vec<String> {
    let lines_unwrapped = line_results
        .into_iter()
        .map(|line| line.as_ref().unwrap().to_string())
        .collect::<Vec<String>>();

    lines_unwrapped
}
