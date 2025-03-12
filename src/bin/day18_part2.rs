use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

use grid::Grid;

#[derive(Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    prev: Option<Box<State>>,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day18.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input, 71, 1024);

    println!("Solution: {solution}");
}

fn solve(input: String, map_size: usize, bytes: usize) -> String {
    let memory: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let mut nums = line.split(",");

            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut map: Grid<char> = Grid::init(map_size, map_size, '.');

    for location in &memory[..bytes.min(memory.len())] {
        map[(location.1, location.0)] = '#';
    }

    let path = find_path(
        &State {
            x: 0,
            y: 0,
            prev: None,
        },
        (map.rows() - 1, map.cols() - 1),
        &map,
    )
    .unwrap();

    let (mut path_locations, mut path_indexes) = recreate_path(path);

    for location in &memory[bytes.min(memory.len())..] {
        map[(location.1, location.0)] = '#';
        if let Some(loc_index) = path_indexes.get(&(location.1, location.0)) {
            if *loc_index == 0 || *loc_index == path_locations.len() - 1 {
                return format!("{},{}", location.0, location.1);
            }

            let start_state = &path_locations[loc_index + 1];

            if let Some(alt_path) = find_path(start_state, (map.rows() - 1, map.cols() - 1), &map) {
                (path_locations, path_indexes) = recreate_path(alt_path);
            } else {
                return format!("{},{}", location.0, location.1);
            }
        }
    }

    String::new()
}

fn find_path(start_state: &State, end_pos: (usize, usize), map: &Grid<char>) -> Option<State> {
    let mut states = VecDeque::new();
    states.push_back(start_state.clone());
    let mut visited_states = HashSet::new();
    visited_states.insert((start_state.x, start_state.y));

    while let Some(state) = states.pop_front() {
        if (state.x, state.y) == end_pos {
            return Some(state);
        }

        states.extend(
            get_next_states(state, &map)
                .into_iter()
                .filter(|s| visited_states.insert((s.x, s.y))),
        );
    }
    None
}

fn recreate_path(state: State) -> (Vec<State>, HashMap<(usize, usize), usize>) {
    let mut curr_state = state;
    let mut path = Vec::new();
    let mut path_locations = HashMap::new();
    path.push(curr_state.clone());
    path_locations.insert((curr_state.x, curr_state.y), 0);
    let mut i = 1;
    while let Some(prev_state) = curr_state.prev {
        path.push(*prev_state.clone());
        path_locations.insert((prev_state.x, prev_state.y), i);
        curr_state = *prev_state;
        i += 1
    }
    (path, path_locations)
}

fn get_next_states(state: State, map: &Grid<char>) -> Vec<State> {
    let up = if state.x > 0 {
        Some((state.x - 1, state.y))
    } else {
        None
    };
    let down = if state.x < map.rows() - 1 {
        Some((state.x + 1, state.y))
    } else {
        None
    };
    let left = if state.y > 0 {
        Some((state.x, state.y - 1))
    } else {
        None
    };
    let right = if state.y < map.cols() - 1 {
        Some((state.x, state.y + 1))
    } else {
        None
    };

    let mut states = vec![];

    match down {
        Some(position) if map[(position.0, position.1)] != '#' => states.push(State {
            x: position.0,
            y: position.1,
            prev: Some(Box::new(state.clone())),
        }),
        _ => {}
    };

    match right {
        Some(position) if map[(position.0, position.1)] != '#' => states.push(State {
            x: position.0,
            y: position.1,
            prev: Some(Box::new(state.clone())),
        }),
        _ => {}
    };

    match up {
        Some(position) if map[(position.0, position.1)] != '#' => states.push(State {
            x: position.0,
            y: position.1,
            prev: Some(Box::new(state.clone())),
        }),
        _ => {}
    };

    match left {
        Some(position) if map[(position.0, position.1)] != '#' => states.push(State {
            x: position.0,
            y: position.1,
            prev: Some(Box::new(state.clone())),
        }),
        _ => {}
    };

    states
}

#[test]
fn test_example() {
    let input = String::from(
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    );

    let solution = solve(input, 7, 12);

    assert_eq!(solution, "6,1");
}
