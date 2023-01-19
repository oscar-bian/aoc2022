use crate::utils::file::*;

pub fn part1(file_name: &String) {
    let contents = get_lines(file_name);
    let lines = unwrap_line_results(&contents);

    let pairs = lines.iter().map(|line| pair_processing(&line));

    let mut subset = 0;

    for pair in pairs {
        let mut was_included = false;

        if pair[0] >= pair[2] {
            if pair[1] <= pair[3] {
                subset += 1;
                was_included = true;
            }
        }

        if pair[2] >= pair[0] {
            if pair[1] >= pair[3] {
                if !was_included {
                    subset += 1;
                }
            }
        }
    }

    println!("{:?}", subset);
}

pub fn part2(file_name: &String) {
    let contents = get_lines(file_name);
    let lines = unwrap_line_results(&contents);
    let pairs = lines.iter().map(|line| pair_processing(&line));

    let mut total = 0;

    for pair in pairs {
        let mut already_included = false;
        if pair[0] <= pair[2] && pair[2] <= pair[1] {
            println!("{:?}", pair);
            total += 1;
            already_included = true;
        }

        if pair[2] <= pair[0] && pair[3] >= pair[0] && !already_included {
            println!("{:?}", pair);
            total += 1;
        }
    }

    println!("{}", total);
}

fn pair_processing(line: &String) -> Vec<i8> {
    line.split(",")
        .map(|f| {
            f.split("-")
                .map(|g| g.trim().parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .flatten()
        .collect::<Vec<i8>>()
}
