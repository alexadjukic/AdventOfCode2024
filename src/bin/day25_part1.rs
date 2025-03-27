use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day25.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let input_iter = input.split("\n\n");
    let mut keys = vec![];
    let mut locks = vec![];

    for input in input_iter {
        let mut lines = input.lines();

        match lines.next().unwrap().chars().next().unwrap() {
            '.' => {
                let mut heights = vec![5, 5, 5, 5, 5];

                for line in lines {
                    for (index, ch) in line.char_indices() {
                        if ch == '.' {
                            heights[index] -= 1;
                        }
                    }
                }
                keys.push(heights);
            }
            '#' => {
                let mut heights = vec![0, 0, 0, 0, 0];
                for line in lines {
                    for (index, ch) in line.char_indices() {
                        if ch == '#' {
                            heights[index] += 1;
                        }
                    }
                }
                locks.push(heights);
            }
            _ => {}
        }
    }

    let mut result = keys.len() * locks.len();

    for key in keys {
        for lock in &locks {
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    result -= 1;
                    break;
                }
            }
        }
    }

    result
}

#[test]
fn test_example() {
    let input = String::from(
        "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
    );

    let solution = solve(input);

    assert_eq!(solution, 3);
}
