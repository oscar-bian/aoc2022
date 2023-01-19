use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn task(file_name: &String) {
    let file_contents = BufReader::new(File::open(file_name).unwrap());

    let mut cycle_count = 1;
    let mut sprite_position = vec![0, 1, 2];
    let mut row_index = 0;
    let mut crt_screen = vec![vec![""; 40]; 6];

    for line_result in file_contents.lines() {
        let line = line_result.unwrap();

        if sprite_position.contains(&((cycle_count - 1) % 40)) {
            crt_screen[row_index][(cycle_count - 1) % 40] = "#"
        } else {
            crt_screen[row_index][(cycle_count - 1) % 40] = "."
        }

        if cycle_count % 40 == 0 {
            row_index += 1
        }
        cycle_count += 1;

        if &line[0..4] == "addx" {
            if sprite_position.contains(&((cycle_count - 1) % 40)) {
                crt_screen[row_index][(cycle_count - 1) % 40] = "#"
            } else {
                crt_screen[row_index][(cycle_count - 1) % 40] = "."
            }

            if cycle_count % 40 == 0 {
                row_index += 1
            }
            cycle_count += 1;
            for i in &mut sprite_position {
                *i = ((*i as i32) + &line[5..].parse::<i32>().unwrap()) as usize;
            }
        }
    }

    for screen_row in &crt_screen {
        println!("{:?}", screen_row.join(""),);
    }
}
