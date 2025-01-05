use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn main() {
    let mut file = File::open("inputs/day7.txt").unwrap();
    let mut input = String::new();
    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve_par(input.clone());

    println!("Solution: {solution}");

    let solution = solve_seq(input);

    println!("Solution: {solution}");
}

fn solve_seq(input: String) -> i64 {
    let equations = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(":");

            let mut equation = VecDeque::new();

            let result = line_iter.next().unwrap().parse::<i64>().unwrap();

            for operand in line_iter.next().unwrap().split(" ").skip(1) {
                equation.push_front(operand.parse::<i64>().unwrap());
            }
            equation.push_front(result);

            equation.into()
        })
        .collect::<Vec<Vec<i64>>>();

    let now = Instant::now();
    let mut calibration_result = 0;
    for equation in equations.into_iter() {
        if is_equation_valid(equation.clone()) {
            calibration_result += equation.first().unwrap();
        }
    }
    println!("Sequential time: {:?}", now.elapsed());

    calibration_result
}

fn solve_par(input: String) -> i64 {
    let equations = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(":");

            let mut equation = VecDeque::new();

            let result = line_iter.next().unwrap().parse::<i64>().unwrap();

            for operand in line_iter.next().unwrap().split(" ").skip(1) {
                equation.push_front(operand.parse::<i64>().unwrap());
            }
            equation.push_front(result);

            equation.into()
        })
        .collect::<Vec<Vec<i64>>>();

    let now = Instant::now();
    let calibration_result = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for equation in equations.into_iter() {
        let result = Arc::clone(&calibration_result);
        let handle = thread::spawn(move || {
            if is_equation_valid(equation.clone()) {
                let mut value = result.lock().unwrap();
                *value += equation.first().unwrap();
            }
        });
        handles.push(handle);
    }
    println!("Parallel time: {:?}", now.elapsed());

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    let x = calibration_result.lock().unwrap();

    *x
}

fn is_equation_valid(mut input: Vec<i64>) -> bool {
    if input.len() == 2 {
        return input[0] == input[1];
    }

    let first = input.pop().unwrap();
    let second = input.pop().unwrap();

    input.push(first * second);
    if is_equation_valid(input.clone()) {
        return true;
    } else {
        input.pop();
    }

    input.push(first + second);
    if is_equation_valid(input.clone()) {
        return true;
    } else {
        input.pop();
    }

    input.push(
        (first.to_string() + &second.to_string())
            .parse::<i64>()
            .unwrap(),
    );
    is_equation_valid(input)
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

    let solution = solve_par(input);

    assert_eq!(solution, 11387);
}
