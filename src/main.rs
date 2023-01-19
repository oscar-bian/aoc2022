use std::env;

mod day_10;
mod day_11;
mod day_12;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let no_file = String::from("no file");

    let day = &args[1];
    let part = &args[2];
    let f = match args.get(3) {
        Some(n) => n,
        _ => &no_file,
    };

    match day.as_str() {
        "10" => {
            let path = format!("./inputs/day_{}/{}.txt", day, f);
            match part.as_str() {
                "1" => day_10::part1::task(&path),
                "2" => day_10::part2::task(&path),
                _ => panic!("not a valid part"),
            }
        }
        "11" => match part.as_str() {
            "1" => day_11::part_1::task(),
            "2" => day_11::part_2::task(),
            _ => panic!("not a valid part"),
        },
        "12" => match part.as_str() {
            "1" => day_12::part_1::task(),
            "2" => day_12::part_2::task(),
            _ => panic!("not a valid part"),
        },
        _ => panic!("day not implemented"),
    }
}
