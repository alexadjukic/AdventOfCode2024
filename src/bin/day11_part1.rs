use std::{
    fs::File,
    io::Read,
    thread::{self, available_parallelism},
    time::Instant,
};

fn main() {
    let mut file = File::open("inputs/day11.txt").unwrap();
    let mut input = String::new();
    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    // let input = String::from("125 17");

    let now = Instant::now();
    let solution = solve_seq(input.clone());
    println!("Solution sequential: {solution}, time: {:?}", now.elapsed());

    let now = Instant::now();
    let solution = solve_par(input);
    println!("Solution parallel: {solution}, time: {:?}", now.elapsed());
}

fn solve_seq(input: String) -> usize {
    let mut input_vec: Vec<u64> = input
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect();

    for _ in 0..25 {
        let temp_vec = input_vec.clone();
        input_vec.clear();

        for item in temp_vec.into_iter() {
            if item == 0 {
                input_vec.push(1);
                continue;
            }

            let digits = get_digits(item);

            if digits.len() % 2 == 0 {
                input_vec.push(join_digits(&digits[..digits.len() / 2]));
                input_vec.push(join_digits(&digits[digits.len() / 2..]));
            } else {
                input_vec.push(item * 2024);
            }
        }
    }

    input_vec.len()
}

fn solve_par(input: String) -> usize {
    let mut input_vec: Vec<u64> = input
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect();

    let num_threads = available_parallelism().unwrap().get();
    let mut handles = Vec::new();

    let mut iter_num = 25;
    loop {
        let temp_vec = input_vec.clone();
        input_vec.clear();
        iter_num -= 1;
        for item in temp_vec.into_iter() {
            if item == 0 {
                input_vec.push(1);
                continue;
            }

            let digits = get_digits(item);

            if digits.len() % 2 == 0 {
                input_vec.push(join_digits(&digits[..digits.len() / 2]));
                input_vec.push(join_digits(&digits[digits.len() / 2..]));
            } else {
                input_vec.push(item * 2024);
            }
        }
        if input_vec.len() >= num_threads {
            break;
        }
    }

    let local_vec_size = input_vec.len() / num_threads;
    for i in 0..num_threads {
        let start_index = local_vec_size * i;
        let end_index = if i == num_threads - 1 {
            input_vec.len()
        } else {
            local_vec_size * (i + 1)
        };
        let local_vec = Vec::from(&input_vec[start_index..end_index]);
        let handle = thread::spawn(move || {
            let mut local_copy = local_vec.clone();
            for _ in 0..iter_num {
                let mut new_vec: Vec<u64> = vec![];

                for item in local_copy.into_iter() {
                    if item == 0 {
                        new_vec.push(1);
                        continue;
                    }

                    let digits = get_digits(item);

                    if digits.len() % 2 == 0 {
                        new_vec.push(join_digits(&digits[..digits.len() / 2]));
                        new_vec.push(join_digits(&digits[digits.len() / 2..]));
                    } else {
                        new_vec.push(item * 2024);
                    }
                }
                local_copy = new_vec;
            }
            local_copy.len()
        });
        handles.push(handle);
    }

    let mut result = 0;
    for handle in handles.into_iter() {
        result += handle.join().unwrap();
    }

    result
}

fn get_digits(mut number: u64) -> Vec<u64> {
    let mut output = Vec::new();

    while number != 0 {
        output.push(number % 10);
        number /= 10;
    }

    output.reverse();
    output
}

fn join_digits(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |acc, num| acc * 10 + num)
}

#[test]
fn text_example() {
    let input = String::from("125 17");

    let solution = solve_seq(input);

    assert_eq!(solution, 55312);
}
