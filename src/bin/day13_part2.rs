use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day13.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> i64 {
    let mut total_tokens = 0;
    for i in (0..input.lines().count()).step_by(4) {
        let lines: Vec<_> = input.lines().collect();
        let button_a = lines[i]
            .split_whitespace()
            .skip(2)
            .filter_map(|word| {
                word.chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .collect::<Vec<i64>>();
        let button_b = lines[i + 1]
            .split_whitespace()
            .skip(2)
            .filter_map(|word| {
                word.chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .collect::<Vec<i64>>();
        let coords = lines[i + 2]
            .split_whitespace()
            .skip(1)
            .filter_map(|word| {
                word.chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse::<i64>()
                    .map(|num| num + 10000000000000)
                    .ok()
            })
            .collect::<Vec<i64>>();

        let determinant = button_a[0] * button_b[1] - button_a[1] * button_b[0];

        if determinant == 0 {
            continue;
        }

        let det_a = coords[0] * button_b[1] - coords[1] * button_b[0];
        let det_b = button_a[0] * coords[1] - button_a[1] * coords[0];

        if det_a % determinant == 0 && det_b % determinant == 0 {
            total_tokens += det_a / determinant * 3 + det_b / determinant;
        }
    }

    total_tokens
}

#[test]
fn test_example() {
    let input = String::from(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    );

    let solution = solve(input);

    assert_eq!(solution, 480);
}
