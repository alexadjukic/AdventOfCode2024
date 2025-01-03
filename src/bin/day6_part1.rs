use std::{collections::HashSet, fs::File, io::Read};

use grid::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait ChangeDirection {
    fn change_direction(&mut self) -> ();
}

impl ChangeDirection for Direction {
    fn change_direction(&mut self) -> () {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        };
    }
}

fn main() {
    //     let input = String::from(
    //         "....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...",
    //     );

    let mut file = File::open("inputs/day6.txt").unwrap();
    let mut input = String::new();
    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("Couldn't read file: {e}");
    }

    let input_grid = Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );
    let mut current_position = (0, 0);
    let mut current_direction = Direction::Up;
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    for (index, location) in input_grid.indexed_iter() {
        if *location == '^' {
            current_position = index;
        } else if *location == '#' {
            obstacles.insert(index);
        }
    }

    loop {
        match current_direction {
            Direction::Up => {
                if current_position.0 == 0 {
                    visited_positions.insert(current_position);
                    break;
                }

                let next_position = (current_position.0 - 1, current_position.1);

                if let Some(_) = obstacles.get(&next_position) {
                    current_direction.change_direction();
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
            Direction::Right => {
                if current_position.1 == input_grid.size().1 - 1 {
                    visited_positions.insert(current_position);
                    break;
                }

                let next_position = (current_position.0, current_position.1 + 1);

                if let Some(_) = obstacles.get(&next_position) {
                    current_direction.change_direction();
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
            Direction::Down => {
                if current_position.0 == input_grid.size().0 - 1 {
                    visited_positions.insert(current_position);
                    break;
                }

                let next_position = (current_position.0 + 1, current_position.1);

                if let Some(_) = obstacles.get(&next_position) {
                    current_direction.change_direction();
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
            Direction::Left => {
                if current_position.1 == 0 {
                    visited_positions.insert(current_position);
                    break;
                }

                let next_position = (current_position.0, current_position.1 - 1);

                if let Some(_) = obstacles.get(&next_position) {
                    current_direction.change_direction();
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
        }
    }
    println!("Solution: {}", visited_positions.len());
}
