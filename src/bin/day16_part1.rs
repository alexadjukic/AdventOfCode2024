use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    usize,
};

use grid::Grid;

enum Move {
    Step,
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
    cost: usize,
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day16.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let map = Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut start_pos = Position {
        x: 0,
        y: 0,
        direction: Direction::Right,
        cost: 0,
    };
    let mut end_pos = (0, 0);

    for (index, elem) in map.indexed_iter() {
        if *elem == 'S' {
            (start_pos.x, start_pos.y) = index;
        } else if *elem == 'E' {
            end_pos = index;
        }
    }

    let mut positions = VecDeque::new();
    positions.push_back(start_pos);

    let mut result = usize::MAX;
    let mut lowest_cost_pos: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some(curr_pos) = positions.pop_front() {
        if (curr_pos.x, curr_pos.y) == end_pos {
            result = result.min(curr_pos.cost);
            continue;
        }

        if result > curr_pos.cost {
            positions.extend(get_moves(&curr_pos, &map).into_iter().filter(|position| {
                *lowest_cost_pos
                    .entry((position.x, position.y))
                    .and_modify(|el| {
                        *el = (*el).min(position.cost);
                    })
                    .or_insert(position.cost)
                    == position.cost
            }));
        }
    }

    result
}

fn get_moves(position: &Position, map: &Grid<char>) -> Vec<Position> {
    let mut moves = vec![];
    let step_forward = get_next_pos(position, &Move::Step);
    if map[(step_forward.x, step_forward.y)] != '#' {
        moves.push(step_forward);
    }

    let step_right = get_next_pos(&get_next_pos(position, &Move::TurnRight), &Move::Step);
    if map[(step_right.x, step_right.y)] != '#' {
        moves.push(step_right);
    }

    let step_left = get_next_pos(&get_next_pos(position, &Move::TurnLeft), &Move::Step);
    if map[(step_left.x, step_left.y)] != '#' {
        moves.push(step_left);
    }

    moves
}

fn get_next_pos(position: &Position, mov: &Move) -> Position {
    match mov {
        Move::Step => match position.direction {
            Direction::Up => Position {
                x: position.x - 1,
                y: position.y,
                direction: position.direction,
                cost: position.cost + 1,
            },
            Direction::Down => Position {
                x: position.x + 1,
                y: position.y,
                direction: position.direction,
                cost: position.cost + 1,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y - 1,
                direction: position.direction,
                cost: position.cost + 1,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y + 1,
                direction: position.direction,
                cost: position.cost + 1,
            },
        },
        Move::TurnLeft => match position.direction {
            Direction::Up => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Left,
                cost: position.cost + 1000,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Right,
                cost: position.cost + 1000,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Down,
                cost: position.cost + 1000,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Up,
                cost: position.cost + 1000,
            },
        },
        Move::TurnRight => match position.direction {
            Direction::Up => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Right,
                cost: position.cost + 1000,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Left,
                cost: position.cost + 1000,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Up,
                cost: position.cost + 1000,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y,
                direction: Direction::Down,
                cost: position.cost + 1000,
            },
        },
    }
}

#[test]
fn test_example1() {
    let input = String::from(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
    );

    let solution = solve(input);

    assert_eq!(solution, 7036);
}

#[test]
fn test_example2() {
    let input = String::from(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    );

    let solution = solve(input);

    assert_eq!(solution, 11048);
}
