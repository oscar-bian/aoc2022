use crate::{
    day_9::{direction::Direction, motion::Motion},
    utils::file::get_lines_v2,
};

use super::{rope::Rope, vis::Vis};

fn parse_motions(lines: &Vec<String>) -> Vec<Motion> {
    lines
        .iter()
        .map(|line| Motion::new(line))
        .collect::<Vec<Motion>>()
}

pub fn part_1(file_name: &String) {
    let lines = get_lines_v2(file_name);

    let set_of_motions = parse_motions(&lines);

    println!("count of motions {:?}", set_of_motions.len());

    let s = set_of_motions
        .clone()
        .into_iter()
        .fold([0, 0, 0, 0], |mut acc, motion| match motion.direction {
            Direction::Down => {
                acc[0] += 1;
                acc
            }
            Direction::Up => {
                acc[1] += 1;
                acc
            }
            Direction::Left => {
                acc[2] += 1;
                acc
            }
            Direction::Right => {
                acc[3] += 1;
                acc
            }
        });

    let grid_size = s.into_iter().max().unwrap() * 2;

    let mut vis = Vis::new(grid_size);

    for motion in set_of_motions {
        vis.process_motion(motion, false);
    }

    println!("{:?}", vis.unique_tail_positions());
}

// guess #1: 6148, too low
// guess #2: 8979, too high

// guess #3: 6175 !!

pub fn part_2(file_name: &String) {
    let lines = get_lines_v2(file_name);
    let set_of_motions = parse_motions(&lines);

    let mut rope = Rope::new(10, 1000);

    for motion in set_of_motions {
        rope.process_motion(&motion);
    }

    println!("{:?}", rope.result());
}
