use std::{fs::File, io::Read};

use grid::Grid;

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: usize,
    prev: Option<Box<State>>,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day20.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let map: Grid<char> = Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    );

    let mut end_pos = (0, 0);
    let mut start_pos = (0, 0);

    for (index, elem) in map.indexed_iter() {
        if *elem == 'E' {
            end_pos = index;
            continue;
        }

        if *elem == 'S' {
            start_pos = index;
        }
    }

    let mut curr_state = State {
        x: start_pos.1,
        y: start_pos.0,
        cost: 0,
        prev: None,
    };

    let mut path: Vec<State> = Vec::new();
    path.push(curr_state.clone());

    loop {
        curr_state = get_next_state(curr_state, &map);
        path.push(curr_state.clone());
        if (curr_state.y, curr_state.x) == end_pos {
            break;
        }
    }

    find_shortcuts(&path)
}

fn find_shortcuts(path: &Vec<State>) -> usize {
    let mut result = 0;
    for i in 0..path.len() {
        for j in (i..path.len()).rev() {
            let start = &path[i];
            let end = &path[j];

            let distance = start.x.abs_diff(end.x) + start.y.abs_diff(end.y);
            let cost_difference = start.cost.abs_diff(end.cost);

            if distance > 20 {
                continue;
            } else if distance == cost_difference {
                break;
            } else if cost_difference.abs_diff(distance) >= 100 {
                result += 1;
            }
        }
    }
    result
}

fn get_next_state(state: State, map: &Grid<char>) -> State {
    let up = (state.y - 1, state.x);
    let down = (state.y + 1, state.x);
    let right = (state.y, state.x + 1);
    let left = (state.y, state.x - 1);

    match state.prev {
        Some(ref prev_state) => {
            if up != (prev_state.y, prev_state.x) && map[up] != '#' {
                State {
                    x: up.1,
                    y: up.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else if down != (prev_state.y, prev_state.x) && map[down] != '#' {
                State {
                    x: down.1,
                    y: down.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else if right != (prev_state.y, prev_state.x) && map[right] != '#' {
                State {
                    x: right.1,
                    y: right.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else {
                State {
                    x: left.1,
                    y: left.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            }
        }
        None => {
            if map[up] != '#' {
                State {
                    x: up.1,
                    y: up.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else if map[down] != '#' {
                State {
                    x: down.1,
                    y: down.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else if map[right] != '#' {
                State {
                    x: right.1,
                    y: right.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            } else {
                State {
                    x: left.1,
                    y: left.0,
                    cost: state.cost + 1,
                    prev: Some(Box::new(state.clone())),
                }
            }
        }
    }
}
