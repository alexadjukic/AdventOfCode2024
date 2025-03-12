use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
};

use grid::Grid;

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

fn solve(input: String, map_size: usize, bytes: usize) -> usize {
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

    let mut states = VecDeque::new();
    states.push_back((0, 0, 0));
    let end_state = (map_size - 1, map_size - 1);
    let mut visited_states = HashSet::new();

    while let Some(state) = states.pop_front() {
        if (state.0, state.1) == end_state {
            return state.2;
        }

        states.extend(
            get_next_states(state, &map)
                .iter()
                .filter(|s| visited_states.insert((s.0, s.1))),
        );
    }
    0
}

fn get_next_states(state: (usize, usize, usize), map: &Grid<char>) -> Vec<(usize, usize, usize)> {
    let up = if state.0 > 0 {
        Some((state.0 - 1, state.1, state.2 + 1))
    } else {
        None
    };
    let down = if state.0 < map.rows() - 1 {
        Some((state.0 + 1, state.1, state.2 + 1))
    } else {
        None
    };
    let left = if state.1 > 0 {
        Some((state.0, state.1 - 1, state.2 + 1))
    } else {
        None
    };
    let right = if state.1 < map.cols() - 1 {
        Some((state.0, state.1 + 1, state.2 + 1))
    } else {
        None
    };

    let mut states = vec![];

    match down {
        Some(state) if map[(state.0, state.1)] != '#' => states.push(state),
        _ => {}
    };

    match right {
        Some(state) if map[(state.0, state.1)] != '#' => states.push(state),
        _ => {}
    };

    match left {
        Some(state) if map[(state.0, state.1)] != '#' => states.push(state),
        _ => {}
    };

    match up {
        Some(state) if map[(state.0, state.1)] != '#' => states.push(state),
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

    assert_eq!(solution, 22);
}
