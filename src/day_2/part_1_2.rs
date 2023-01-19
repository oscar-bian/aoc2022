use std::fs::{self, File};
use std::io::{BufReader, Error, Read};
// use std::path::Path;

fn main() {
    /*
        A, X: Rock, bonus = 1
        B, Y: Paper, bonus = 2
        C, Z: Scissors, bonus = 3
        Draw 3
        win 6
        loss 0

        puzzle 2

        X lose
        Y draw
        Z win
    */

    let file_name = String::from("input_1.txt");
    let lines_maybe = get_lines(&file_name);
    let mut score = 0;
    for line_result in lines_maybe {
        let line = line_result.unwrap();
        let split = line.split_whitespace().collect::<Vec<&str>>();
        match split[0] {
            "A" => match split[1] {
                "X" => score += 0 + 3,
                "Y" => score += 3 + 1,
                "Z" => score += 6 + 2,
                _ => println!("Oops"),
            },
            "B" => match split[1] {
                "X" => score += 0 + 1,
                "Y" => score += 3 + 2,
                "Z" => score += 6 + 3,
                _ => println!("Oops"),
            },
            "C" => match split[1] {
                "X" => score += 0 + 2,
                "Y" => score += 3 + 3,
                "Z" => score += 6 + 1,
                _ => println!("Oops"),
            },
            _ => println!("Oops"),
        }
    }
    println!("Score: {:?}", score);
}

// fn read_lines(path: &Path) -> String {
//     let mut f = File::open(path).expect(&format!("file not found: {}", path.display()));

//     let mut contents = String::new();

//     f.read_to_string(&mut contents)
//         .expect(&format!("cannot read file {}", path.display()));

//     contents
// }

fn get_lines(file_name: &String) -> Vec<Result<String, Error>> {
    let mut lines = Vec::new();

    lines =
        std::io::BufRead::lines(BufReader::new(File::open(file_name).unwrap())).collect::<Vec<_>>();
    lines
}
