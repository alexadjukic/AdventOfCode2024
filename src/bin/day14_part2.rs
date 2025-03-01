use std::{
    collections::HashMap,
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use regex::Regex;
use ring_algorithm::chinese_remainder_theorem;

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

fn solve(input: String, board_size: (i32, i32)) -> i32 {
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

    let mut max_x = 0;
    let mut max_y = 0;
    let mut iter_x = 1;
    let mut iter_y = 1;
    for i in 1..=103 {
        let mut robot_map: HashMap<(i32, i32), usize> = HashMap::new();
        let mut x_map: HashMap<i32, usize> = HashMap::new();
        let mut y_map: HashMap<i32, usize> = HashMap::new();
        for robot in robots.iter() {
            let mut final_x = (robot.0 + i * robot.2) % board_size.0;
            let mut final_y = (robot.1 + i * robot.3) % board_size.1;

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

            robot_map
                .entry((final_x, final_y))
                .and_modify(|e| *e += 1)
                .or_insert(1);

            x_map.entry(final_x).and_modify(|e| *e += 1).or_insert(1);
            y_map.entry(final_y).and_modify(|e| *e += 1).or_insert(1);
        }

        let curr_x = *x_map.values().max().unwrap() as i32;

        (max_x, iter_x) = if max_x < curr_x {
            (curr_x, i)
        } else {
            (max_x, iter_x)
        };

        let curr_y = *y_map.values().max().unwrap() as i32;

        (max_y, iter_y) = if max_y < curr_y {
            (curr_y, i)
        } else {
            (max_y, iter_y)
        };
    }

    let remainders = [iter_x, iter_y];
    let modulos = [101, 103];

    let solution = chinese_remainder_theorem(&remainders, &modulos).unwrap();

    visualize(board_size, robots);

    solution
}

fn visualize(board_size: (i32, i32), robots: Vec<(i32, i32, i32, i32)>) {
    let mut iter_counter = 0;
    loop {
        print!(
            "Menu
    [Enter] - 1 iteration forward
    [X] - X iterations forward
    [q] - exit
    Choose: "
        );
        let _ = stdout().flush();
        let mut signal = String::new();
        stdin().read_line(&mut signal).unwrap();
        let signal = signal.trim();

        if signal == "q" {
            break;
        } else if let Ok(val) = signal.parse::<i32>() {
            iter_counter += val;
        } else {
            iter_counter += 1;
        }

        let mut robot_map: HashMap<(i32, i32), usize> = HashMap::new();
        let mut x_map: HashMap<i32, usize> = HashMap::new();
        let mut y_map: HashMap<i32, usize> = HashMap::new();
        for robot in robots.iter() {
            let mut final_x = (robot.0 + iter_counter * robot.2) % board_size.0;
            let mut final_y = (robot.1 + iter_counter * robot.3) % board_size.1;

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

            robot_map
                .entry((final_x, final_y))
                .and_modify(|e| *e += 1)
                .or_insert(1);

            x_map.entry(final_x).and_modify(|e| *e += 1).or_insert(1);
            y_map.entry(final_y).and_modify(|e| *e += 1).or_insert(1);
        }

        println!("Iteration: {iter_counter}");
        for i in 0..103 {
            for j in 0..101 {
                if let Some(char) = robot_map.get(&(j, i)) {
                    print!("{char}");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
