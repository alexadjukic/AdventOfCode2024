use std::{
    collections::{vec_deque, HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

use grid::Grid;

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day15.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(input: String) -> usize {
    let mut input_iter = input.split("\n\n");

    let mut map = Grid::from(
        input_iter
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|ch| {
                        if ch == '#' {
                            "##".chars()
                        } else if ch == 'O' {
                            "[]".chars()
                        } else if ch == '.' {
                            "..".chars()
                        } else {
                            "@.".chars()
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>(),
    );

    let moves: Vec<Direction> = input_iter
        .next()
        .unwrap()
        .chars()
        .filter_map(|ch| {
            if ch == '^' {
                Some(Direction::Up)
            } else if ch == 'v' {
                Some(Direction::Down)
            } else if ch == '<' {
                Some(Direction::Left)
            } else if ch == '>' {
                Some(Direction::Right)
            } else {
                None
            }
        })
        .collect();

    let mut curr_pos = (0, 0);

    for (index, elem) in map.indexed_iter() {
        if *elem == '@' {
            curr_pos = index;
            break;
        }
    }

    for next_move in moves {
        execute_move(&mut curr_pos, &mut map, next_move);
    }

    let mut result = 0;
    for (index, elem) in map.indexed_iter() {
        if *elem == '[' {
            result += index.0 * 100 + index.1;
        }
    }

    result
}

fn execute_move(current_pos: &mut (usize, usize), map: &mut Grid<char>, direction: Direction) {
    let mut pending_changes: HashMap<(usize, usize), char> = HashMap::new();
    let mut next_pos = *current_pos;

    if direction == Direction::Left || direction == Direction::Right {
        let mut last_char = '@';
        loop {
            next_pos = get_next_position(next_pos, direction);
            let next_symbol = map[next_pos];
            if next_symbol == '#' {
                return;
            } else if next_symbol == '.' {
                pending_changes.insert(next_pos, last_char);
                pending_changes.insert(*current_pos, '.');
                break;
            } else {
                pending_changes.insert(next_pos, last_char);
                last_char = next_symbol;
            }
        }
    } else {
        let mut verticals = VecDeque::new();
        verticals.push_back((current_pos.0, current_pos.1, '@'));
        let mut visited_locations = HashSet::new();
        while !verticals.is_empty() {
            let vertical = verticals.pop_front().unwrap();
            let mut last_char = vertical.2;
            next_pos = (vertical.0, vertical.1);
            if !visited_locations.insert(next_pos) {
                continue;
            }

            loop {
                next_pos = get_next_position(next_pos, direction);
                let next_symbol = map[next_pos];
                visited_locations.insert(next_pos);
                if next_symbol == '#' {
                    return;
                } else if next_symbol == '.' {
                    pending_changes.insert(next_pos, last_char);
                    pending_changes.insert((vertical.0, vertical.1), '.');
                    break;
                } else if next_symbol == '[' {
                    pending_changes.insert(next_pos, last_char);
                    last_char = next_symbol;
                    let right_half = get_next_position(next_pos, Direction::Right);
                    verticals.push_back((right_half.0, right_half.1, ']'));
                } else if next_symbol == ']' {
                    pending_changes.insert(next_pos, last_char);
                    last_char = next_symbol;
                    let left_half = get_next_position(next_pos, Direction::Left);
                    verticals.push_back((left_half.0, left_half.1, '['));
                }
            }
        }
    }

    *current_pos = get_next_position(*current_pos, direction);

    for change in pending_changes {
        map[change.0] = change.1
    }
}

fn get_next_position(position: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (position.0 - 1, position.1),
        Direction::Down => (position.0 + 1, position.1),
        Direction::Left => (position.0, position.1 - 1),
        Direction::Right => (position.0, position.1 + 1),
    }
}

// #[test]
// fn text_example1() {
//     let input = String::from(
//         "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
//     );

//     let solution = solve(input);

//     assert_eq!(solution, 9021);
// }

// ##......##..##......##..##..[]....[]........[]##[]....[]....[][]..[]....[]........[]##....[]......##
// ##..[]##..[]........[]....[]..##[]....[]..[][].[][]...............[]..[]................[]....[][]##
// ##....[][]##[]....[]....####..[][]........[]..[]..##......[]........[]..........[]..[]......##..[]##
// ##........[]......[]........[][]..##.....[][].[]..................[]......[]..........[][]......####
// ##..[]##..##[]..[]....[]..##[][][].....[][][][][][]...[]........##..[]..........[]....[]..##......##
// ##..[]..[][][].................[]........[].[]..[]..##........[]....##........[]........[]....[]..##
// ##[][]..[]..........##..##.......[]..........[]...........##....##....[]....##..........####....[]##
// ##......##..[].........[]....................@..[][]..........##....[]......##......[]........[]..##
// ##..[]..[]..[]................[]........[]......##[]....[]..##..[]....##....[][][]........[]......##
#[test]
fn text_example2() {
    let input = String::from(
        "#############
#...........#
#...O#O.....#
#..OO.OO....#
#..O.O......#
#@OO.O......#
#....OOOOOO.#
#...O.......#
#....O......#
#...........#
#############

>>vvvv>>>>>>>>>>>>>>>>>>>^^^^^^<<<<<<<<>>>>>>>vvv<<<<<<<vv<<<<v<<^^^",
    );

    let solution = solve(input);

    assert_eq!(solution, 9021);
}
