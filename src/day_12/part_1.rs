use std::collections::HashMap;

struct Vertex {
    coordinates: (usize, usize),
    height: usize,
    letter: String,
}

impl Vertex {
    fn new(coordinates: (usize, usize), height: usize, letter: String) -> Self {
        Vertex {
            coordinates,
            height,
            letter,
        }
    }
}

struct Graph {
    vertices: Vec<Vertex>,
    edges: HashMap<(usize, usize), (usize, usize)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    fn add_edge(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.edges.insert(from, to);
    }
}

pub enum Neighbors {
    Up,
    Down,
    Left,
    Right,
}

pub fn find_eligible_neighbors(
    coordinates: (usize, usize),
    grid: &Vec<Vec<usize>>,
) -> Vec<Neighbors> {
    let mut neigbhors: Vec<Neighbors> = Vec::new();

    let (x, y) = coordinates;

    let current_height = grid[y][x];

    if x != 0 && (grid[x - 1][y] - current_height) < 1 {
        neigbhors.push(Neighbors::Left);
    }

    if x != grid.len() - 1 && (grid[x + 1][y] - current_height) < 1 {
        neigbhors.push(Neighbors::Right);
    }

    if y != 0 && (grid[x][y - 1] - current_height) < 1 {
        neigbhors.push(Neighbors::Down);
    }

    if y != grid.len() - 1 && (grid[x][y + 1] - current_height) < 1 {
        neigbhors.push(Neighbors::Up);
    }

    neigbhors
}

pub fn task() {
    let mut all_points: Vec<Vertex> = Vec::new();

    let lower_case_alphabet = ('a'..='z').into_iter().collect::<String>();

    let mut start_coordinate = (0, 0);
    let mut end_coordinate = (0, 0);

    let grid: Vec<Vec<usize>> = include_str!("../../inputs/day_12/example.txt")
        .split("\n")
        .enumerate()
        .map(|(line_counter, line)| {
            let heights = line
                .chars()
                .enumerate()
                .map(|(character_counter, character)| {
                    let possible_height = lower_case_alphabet.find(character);
                    match possible_height {
                        Some(height) => {
                            all_points.push(Vertex::new(
                                (character_counter, line_counter),
                                height,
                                character.to_string(),
                            ));
                            height
                        }
                        None => match character {
                            _ if character.to_string() == "S" => {
                                start_coordinate.0 = character_counter;
                                start_coordinate.1 = line_counter;
                                10
                            }
                            _ if character.to_string() == "E" => {
                                end_coordinate.0 = character_counter;
                                end_coordinate.1 = line_counter;
                                100
                            }
                            _ => panic!("unhandled"),
                        },
                    }
                })
                .collect::<Vec<usize>>();

            heights
        })
        .collect();

    for col in &grid {
        println!(
            "{:?}",
            col.iter()
                .map(|item| if item < &10 {
                    format!("  {:?}", item)
                } else if item == &100 {
                    format!("{:?}", item)
                } else {
                    format!(" {:?}", item)
                })
                .collect::<String>()
        );
    }

    println!("{:?}, {:?}", start_coordinate, end_coordinate);

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut goal_reached = false;

    let mut current_coordinates = start_coordinate.clone();

    while !goal_reached {
        let mut current_neighbors = find_eligible_neighbors(current_coordinates, &grid);

        for current_neighbor in current_neighbors {
            match current_neighbor {
                Neighbors::Down => {}
                Neighbors::Up => {}
                Neighbors::Right => {}
                Neighbors::Left => {}
            }
        }
    }
}
