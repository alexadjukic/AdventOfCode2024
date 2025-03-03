use std::{fs::File, io::Read};

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

#[derive(Debug)]
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
            .map(|line| line.chars().collect())
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
        if *elem == 'O' {
            result += index.0 * 100 + index.1;
        }
    }

    result
}

fn execute_move(current_pos: &mut (usize, usize), map: &mut Grid<char>, direction: Direction) {
    let mut last_pos = *current_pos;
    let mut pos_history = vec![];
    loop {
        last_pos = match direction {
            Direction::Up => (last_pos.0 - 1, last_pos.1),
            Direction::Down => (last_pos.0 + 1, last_pos.1),
            Direction::Left => (last_pos.0, last_pos.1 - 1),
            Direction::Right => (last_pos.0, last_pos.1 + 1),
        };
        pos_history.push(last_pos);
        let curr_symbol = map[last_pos];

        if curr_symbol == '#' {
            break;
        } else if curr_symbol == '.' {
            let next_pos = *pos_history.first().unwrap();
            map[last_pos] = 'O';
            map[next_pos] = '@';
            map[*current_pos] = '.';
            current_pos.0 = next_pos.0;
            current_pos.1 = next_pos.1;
            break;
        }
    }
}

#[test]
fn text_example1() {
    let input = String::from(
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    );

    let solution = solve(input);

    assert_eq!(solution, 10092);
}

#[test]
fn test_example2() {
    let input = String::from(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    );

    let solution = solve(input);

    assert_eq!(solution, 2028);
}
