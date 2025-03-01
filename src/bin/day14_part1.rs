use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day14.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input, (101, 103));

    println!("Solution: {solution}");
}

fn solve(input: String, board_size: (i32, i32)) -> usize {
    let regex = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut robots = vec![];
    for captures in regex.captures_iter(&input) {
        let (_, [px, py, vx, vy]) = captures.extract();
        robots.push((
            px.parse::<i32>().unwrap(),
            py.parse::<i32>().unwrap(),
            vx.parse::<i32>().unwrap(),
            vy.parse::<i32>().unwrap(),
        ));
    }

    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        let mut final_x = (robot.0 + 100 * robot.2) % board_size.0;
        let mut final_y = (robot.1 + 100 * robot.3) % board_size.1;

        final_x = if final_x < 0 {
            final_x + board_size.0
        } else {
            final_x
        };
        final_y = if final_y < 0 {
            final_y + board_size.1
        } else {
            final_y
        };

        let mid_x = board_size.0 / 2;
        let mid_y = board_size.1 / 2;

        let index_x = if final_x < mid_x {
            Some(0b00)
        } else if final_x > mid_x {
            Some(0b01)
        } else {
            None
        };

        let index_y = if final_y < mid_y {
            Some(0b00)
        } else if final_y > mid_y {
            Some(0b10)
        } else {
            None
        };

        if let (Some(x_quad), Some(y_quad)) = (index_x, index_y) {
            quadrants[x_quad | y_quad] += 1;
        }
    }
    quadrants.iter().product()
}

#[test]
fn test_example() {
    let input = String::from(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    );

    let solution = solve(input, (11, 7));

    assert_eq!(solution, 12);
}
