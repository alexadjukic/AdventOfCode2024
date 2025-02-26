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

fn solve(input: String) -> i32 {
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
            .collect::<Vec<i32>>();
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
            .collect::<Vec<i32>>();
        let coords = lines[i + 2]
            .split_whitespace()
            .skip(1)
            .filter_map(|word| {
                word.chars()
                    .filter(|char| char.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .collect::<Vec<i32>>();

        let mut best_solution = 0;
        for j in (1..=100).rev() {
            let remainder_x = coords[0] - button_b[0] * j;
            let remainder_y = coords[1] - button_b[1] * j;

            if remainder_x % button_a[0] == 0 && remainder_y % button_a[1] == 0 {
                let coef1 = remainder_x / button_a[0];
                let coef2 = remainder_y / button_a[1];
                if coef1 == coef2 && coef1 <= 100 {
                    let solution = coef1 * 3 + j;
                    if best_solution == 0 || solution < best_solution {
                        best_solution = solution;
                    }
                }
            }
        }
        total_tokens += best_solution;
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
