use std::collections::HashSet;

use super::{direction::Direction, motion::Motion, position::Position};

#[derive(Debug, PartialEq, Clone)]
pub struct Vis {
    grid: Vec<Vec<String>>,
    all_head_positions: Vec<Position>,
    pub all_tail_positions: Vec<Position>,
    current_head_position: Position,
    current_tail_position: Position,
}

impl Vis {
    pub fn new(size: usize) -> Vis {
        let mut new_grid: Vec<Vec<String>> = Vec::new();

        let center = (size / 2) as i16;

        for _ in 0..size {
            new_grid.push(vec![".".to_string(); size.into()]);
        }
        let new_current_head_position = Position::new(center, center);
        let new_current_tail_position = Position::new(center, center);
        Vis {
            grid: new_grid,
            all_head_positions: vec![new_current_head_position],
            all_tail_positions: vec![new_current_head_position],
            current_head_position: new_current_head_position,
            current_tail_position: new_current_tail_position,
        }
    }

    pub fn process_motion(&mut self, motion: Motion, print_grid: bool) {
        for _ in 0..motion.steps {
            self.update_head_position(motion.direction);
            if self.should_tail_update() {
                self.update_tail_position();
            }
            if print_grid {
                self.print(&motion);
            }
        }
    }

    fn should_tail_update(&mut self) -> bool {
        f64::sqrt(
            (u16::pow(
                i32::abs((self.current_head_position.x - self.current_tail_position.x).into())
                    .try_into()
                    .unwrap(),
                2,
            ) + u16::pow(
                i32::abs((self.current_head_position.y - self.current_tail_position.y).into())
                    .try_into()
                    .unwrap(),
                2,
            )) as f64,
        ) > f64::sqrt(2.0)
    }

    fn update_tail_position(&mut self) {
        self.update_grid(
            self.current_tail_position.x,
            self.current_tail_position.y,
            ".",
        );
        let n = self.all_head_positions.len() - 2;
        let previous_head_position = self.all_head_positions[n];
        self.current_tail_position = previous_head_position.clone();
        self.all_tail_positions
            .push(self.current_tail_position.clone());
    }

    fn update_head_position(&mut self, direction: Direction) {
        self.update_grid(
            self.current_head_position.x,
            self.current_head_position.y,
            ".",
        );
        self.current_head_position.update(direction, 1);
        self.all_head_positions
            .push(self.current_head_position.clone());
    }

    fn update_grid(&mut self, x_index: i16, y_index: i16, character: &str) {
        let y: u16 = y_index.try_into().unwrap();
        let x: u16 = x_index.try_into().unwrap();
        self.grid[usize::from(y)][usize::from(x)] = character.to_string();
    }

    fn print(&mut self, motion: &Motion) {
        println!("{:?}", motion);
        self.update_grid(
            self.current_tail_position.x,
            self.current_tail_position.y,
            "T",
        );
        self.update_grid(
            self.current_head_position.x,
            self.current_head_position.y,
            "H",
        );
        for (i, line) in self.grid.clone().into_iter().enumerate().rev() {
            println!("{:?} {:?}", i, line.join(" "));
        }
        print!("\n");
    }

    pub fn unique_tail_positions(&mut self) -> usize {
        let mut n: HashSet<(i16, i16)> = HashSet::new();

        for i in &self.all_tail_positions {
            n.insert((i.x, i.y));
        }

        n.len()
    }
}
