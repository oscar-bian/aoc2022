use std::collections::HashSet;

use super::{direction::Direction, motion::Motion};

#[derive(Debug, Clone)]
pub struct Rope {
    pub knots: Vec<(usize, usize)>,
    length: usize,
    grid_size: usize,
    tail_visited_positions: HashSet<(usize, usize)>,
    grid: Vec<Vec<String>>,
}

impl Rope {
    fn init_grid(n: usize) -> Vec<Vec<String>> {
        let mut new_grid = Vec::new();
        for _ in 0..n {
            new_grid.push(vec![".".to_string(); n])
        }
        new_grid
    }

    pub fn new(length: usize, grid_size: usize) -> Rope {
        let k = grid_size / 2;
        let knots = vec![(k, k); length];
        Rope {
            knots,
            length,
            tail_visited_positions: HashSet::new(),
            grid: Rope::init_grid(grid_size),
            grid_size,
        }
    }

    fn getter(&self) -> Vec<(usize, usize)> {
        let mut knots_copy: Vec<(usize, usize)> = Vec::new();
        for (x, y) in self.knots.iter() {
            knots_copy.push((x.clone(), y.clone()));
        }

        knots_copy
    }

    pub fn process_motion(&mut self, motion: &Motion) {
        let mut h: HashSet<String> = HashSet::new();
        for _ in 0..motion.steps {
            self.clean_grid();
            for knot_index in 0..self.length {
                // the first knot is the head
                if knot_index == 0 {
                    self.update_first_knot(&motion.direction);
                // all other knots are updated relative to each other
                } else {
                    let previous_knot_index = knot_index - 1;
                    let current_knot_index = knot_index;

                    let knot_distance =
                        self.get_knot_distance(previous_knot_index, current_knot_index);

                    h.insert(knot_distance.to_string());
                    // if the distance is sqrt(5) we need a diagonal update
                    if knot_distance >= f64::sqrt(5.0) {
                        self.diagonal_update(previous_knot_index, current_knot_index);
                    } else if knot_distance > f64::sqrt(2.0) {
                        self.straight_update(previous_knot_index, current_knot_index)
                    }
                    // if the distance is sqrt(2) or less, the knots are touching or overlapping and should need be updated

                    // always update the last move
                    self.tail_visited_positions
                        .insert(self.knots[self.length - 1].clone());
                }
            }
        }
        // self.print_grid();
    }

    fn clean_grid(&mut self) {
        for knot in self.knots.iter().rev() {
            let (x, y) = knot.clone();
            self.grid[y][x] = ".".to_string();
        }
    }

    fn print_grid(&mut self) {
        self.grid[self.grid_size / 2][self.grid_size / 2] = "s".to_string();
        for (idx, knot) in self.knots.iter().enumerate().rev() {
            let mut marker = idx.to_string();
            if idx == 0 {
                marker = "H".to_string();
            }
            let (x, y) = knot.clone();
            self.grid[y][x] = marker;
        }
        for row in self.grid.iter().rev() {
            println!("{:?}", row.join(""));
        }
        println!("\n")
    }

    fn update_first_knot(&mut self, direction: &Direction) {
        self.update_knot(0, direction)
    }

    fn straight_update(&mut self, first_knot_index: usize, second_knot_index: usize) {
        let (first_knot_x, first_knot_y) = self.knots[first_knot_index];
        let (second_knot_x, second_knot_y) = self.knots[second_knot_index];

        if first_knot_x == second_knot_x {
            // H.
            // ..
            // T.
            if first_knot_y > second_knot_y {
                self.knots[second_knot_index].1 += 1;
            }
            // T.
            // ..
            // H.
            if first_knot_y < second_knot_y {
                self.knots[second_knot_index].1 -= 1;
            }
        }

        if first_knot_y == second_knot_y {
            // ...
            // T.H
            // ...
            if first_knot_x > second_knot_x {
                self.knots[second_knot_index].0 += 1;
            }
            // ...
            // H.T
            // ..
            if first_knot_x < second_knot_x {
                self.knots[second_knot_index].0 -= 1;
            }
        }
    }

    fn diagonal_update(&mut self, first_knot_index: usize, second_knot_index: usize) {
        let (first_knot_x, first_knot_y) = self.knots[first_knot_index];
        let (second_knot_x, second_knot_y) = self.knots[second_knot_index];

        if first_knot_x > second_knot_x {
            // .H
            // ..
            // T.
            if first_knot_y > second_knot_y {
                self.knots[second_knot_index].0 += 1;
                self.knots[second_knot_index].1 += 1;
            // T.
            // ..
            // .H
            } else {
                self.knots[second_knot_index].0 += 1;
                self.knots[second_knot_index].1 -= 1;
            }
        } else {
            // H.
            // ..
            // .T
            if first_knot_y > second_knot_y {
                self.knots[second_knot_index].0 -= 1;
                self.knots[second_knot_index].1 += 1;
                // .T
                // ..
                // H.
            } else {
                self.knots[second_knot_index].0 -= 1;
                self.knots[second_knot_index].1 -= 1;
            }
        }
    }

    fn update_knot(&mut self, knot_index: usize, direction: &Direction) {
        match direction {
            Direction::Down => self.knots[knot_index].1 -= 1,
            Direction::Up => self.knots[knot_index].1 += 1,
            Direction::Right => self.knots[knot_index].0 += 1,
            Direction::Left => self.knots[knot_index].0 -= 1,
        }
    }

    pub fn get_knot_distance(&mut self, first_knot_index: usize, second_knot_index: usize) -> f64 {
        let (first_knot_x, first_knot_y) = self.knots[first_knot_index];
        let (second_knot_x, second_knot_y) = self.knots[second_knot_index];

        let x1: i32 = first_knot_x.try_into().unwrap();
        let y1: i32 = first_knot_y.try_into().unwrap();
        let x2: i32 = second_knot_x.try_into().unwrap();
        let y2: i32 = second_knot_y.try_into().unwrap();

        let res1 = f64::sqrt(f64::powi((x1 - x2) as f64, 2) + f64::powi((y1 - y2) as f64, 2));

        res1
    }

    pub fn result(&self) -> usize {
        self.tail_visited_positions.len()
    }
}
