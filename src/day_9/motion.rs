use std::str::FromStr;

use crate::day_9::direction::Direction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Motion {
    pub direction: Direction,
    pub steps: i16,
}

impl Motion {
    pub fn new(line: &String) -> Motion {
        let l = line.split_whitespace().collect::<Vec<_>>();
        Motion {
            direction: Direction::from_str(l[0]).unwrap(),
            steps: l[1].parse::<i16>().unwrap(),
        }
    }
}
