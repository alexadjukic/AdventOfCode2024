use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use grid::*;

#[derive(Debug)]
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
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    for (index, location) in input_grid.indexed_iter() {
        if *location == '^' {
            current_position = index;
        } else if *location == '#' {
            obstacles.insert(index);
        }
    }

    traverse(
        &input_grid,
        current_position,
        &obstacles,
        &mut visited_positions,
    );

    test_seq(
        &input_grid,
        current_position,
        &obstacles,
        &visited_positions,
    );

    test_par(
        &input_grid,
        current_position,
        &obstacles,
        &visited_positions,
    );
}

fn test_seq(
    input_grid: &Grid<char>,
    current_position: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    visited_positions: &HashSet<(usize, usize)>,
) {
    let now = Instant::now();
    let mut new_obstacles_count = 0;
    for position in visited_positions.iter() {
        let mut new_obstacle_set = obstacles.clone();
        new_obstacle_set.insert(*position);
        new_obstacles_count +=
            traverse_indefinitely_seq(&input_grid, current_position, &new_obstacle_set);
    }
    let elapsed = now.elapsed();
    println!(
        "Sequential execution time: {elapsed:?}. Checked positions: {}",
        visited_positions.len()
    );
    println!("Solution: {new_obstacles_count}");
}

fn test_par(
    input_grid: &Grid<char>,
    current_position: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    visited_positions: &HashSet<(usize, usize)>,
) {
    let now = Instant::now();
    let new_obstacles_count = Arc::new(Mutex::new(0));
    let mut thread_handles = Vec::new();
    for position in visited_positions.iter() {
        let mut new_obstacle_set = obstacles.clone();
        new_obstacle_set.insert(*position);
        let obstacle_count_copy = Arc::clone(&new_obstacles_count);
        let input_grid_copy = input_grid.clone();
        let current_position_copy = current_position.clone();
        let handle = thread::spawn(move || {
            traverse_indefinitely_par(
                input_grid_copy,
                current_position_copy,
                &new_obstacle_set,
                obstacle_count_copy,
            )
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles.into_iter() {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed();
    println!(
        "Parallel execution time: {elapsed:?}. Checked positions: {}",
        visited_positions.len()
    );
    let solution = new_obstacles_count.lock().unwrap();
    println!("Solution: {solution}");
}

fn traverse(
    input_grid: &Grid<char>,
    mut current_position: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    visited_positions: &mut HashSet<(usize, usize)>,
) {
    let mut current_direction = Direction::Up;
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
}

fn traverse_indefinitely_seq(
    input_grid: &Grid<char>,
    mut current_position: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
) -> usize {
    let mut current_direction = Direction::Up;
    let mut visited_positions = HashSet::new();
    let mut loop_counter = 0;
    let mut visited_positions_counter = visited_positions.len();
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            return 1;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            return 1;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            return 1;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            return 1;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
        }
    }
    0
}

fn traverse_indefinitely_par(
    input_grid: Grid<char>,
    mut current_position: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    added_obstacles: Arc<Mutex<usize>>,
) {
    let mut current_direction = Direction::Up;
    let mut visited_positions = HashSet::new();
    let mut loop_counter = 0;
    let mut visited_positions_counter = visited_positions.len();
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            let mut value = added_obstacles.lock().unwrap();
                            *value += 1;
                            return;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            let mut value = added_obstacles.lock().unwrap();
                            *value += 1;
                            return;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            let mut value = added_obstacles.lock().unwrap();
                            *value += 1;
                            return;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
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
                    if visited_positions_counter == visited_positions.len() {
                        loop_counter += 1;
                        if loop_counter == 2 {
                            let mut value = added_obstacles.lock().unwrap();
                            *value += 1;
                            return;
                        }
                    } else {
                        loop_counter = 0;
                        visited_positions_counter = visited_positions.len();
                    }
                } else {
                    visited_positions.insert(current_position);
                    current_position = next_position;
                }
            }
        }
    }
}
