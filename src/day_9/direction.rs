use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
