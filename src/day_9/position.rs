use super::direction::Direction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Position { x, y }
    }

    pub fn update(&mut self, direction: Direction, step: i16) {
        match direction {
            Direction::Right => self.x += step,
            Direction::Left => self.x -= step,
            Direction::Up => self.y += step,
            Direction::Down => self.y -= step,
        }
    }
}
