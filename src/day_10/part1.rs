use std::fs;

pub fn task(file_name: &String) {
    let file_contents = fs::read_to_string(file_name).unwrap();

    let mut x = 1;
    let mut signal_sum = 0;

    let mut cycle_count = 1;

    for line_content in file_contents.lines() {
        cycle_count += 1;

        if cycle_count == 20 || cycle_count % 40 == 20 {
            signal_sum += x * i32::try_from(cycle_count).unwrap();
        }

        if &line_content[0..4] == "addx" {
            cycle_count += 1;
            x += line_content[5..].parse::<i32>().unwrap();

            if cycle_count == 20 || cycle_count % 40 == 20 {
                signal_sum += x * i32::try_from(cycle_count).unwrap();
            }
        }
    }

    println!("{:?}", signal_sum);
}
