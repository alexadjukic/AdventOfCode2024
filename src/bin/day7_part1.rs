use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day7.txt").unwrap();
    let mut input = String::new();
    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> i64 {
    let equations = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(":");

            let mut equation = Vec::new();

            equation.push(line_iter.next().unwrap().parse::<i64>().unwrap());

            for operand in line_iter.next().unwrap().split(" ").skip(1) {
                equation.push(operand.parse::<i64>().unwrap());
            }

            equation
        })
        .collect::<Vec<Vec<i64>>>();

    let mut calibration_result = 0;
    for equation in equations.iter() {
        if is_equation_valid(equation.clone()) {
            calibration_result += equation[0];
        }
    }

    calibration_result
}

fn is_equation_valid(mut input: Vec<i64>) -> bool {
    if input.len() == 2 {
        return input[0] == input[1];
    }

    let last = input.pop().unwrap();
    let mut input_sub = input.clone();

    if input[0] % last == 0 {
        input[0] /= last;
        if is_equation_valid(input.clone()) {
            return true;
        }
    }

    input_sub[0] -= last;
    is_equation_valid(input_sub)
}

#[test]
fn test_example() {
    let input = String::from(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    );

    let solution = solve(input);

    assert_eq!(solution, 3749);
}
