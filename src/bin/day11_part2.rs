use std::{collections::HashMap, fs::File, io::Read, time::Instant};

fn main() {
    let mut file = File::open("inputs/day11.txt").unwrap();
    let mut input = String::new();
    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let now = Instant::now();
    let solution = solve_seq(input.clone());
    println!("Solution: {solution}, time: {:?}", now.elapsed());
}

fn traverse_tree(node: u64, depth: usize, map: &mut HashMap<(u64, usize), usize>) -> usize {
    if depth == 0 {
        return 1;
    }

    match map.get(&(node, depth)) {
        Some(value) => {
            return *value;
        }
        None => {
            if node == 0 {
                let result = traverse_tree(1, depth - 1, map);
                map.insert((node, depth), result);
                return result;
            }

            let digits = get_digits(node);

            if digits.len() % 2 == 0 {
                let new_elem1 = join_digits(&digits[..digits.len() / 2]);
                let new_elem2 = join_digits(&digits[digits.len() / 2..]);
                let result = traverse_tree(new_elem1, depth - 1, map)
                    + traverse_tree(new_elem2, depth - 1, map);
                map.insert((node, depth), result);
                return result;
            } else {
                let new_elem = node * 2024;
                let result = traverse_tree(new_elem, depth - 1, map);
                map.insert((node, depth), result);
                return result;
            }
        }
    }
}

fn solve_seq(input: String) -> usize {
    let input_vec: Vec<u64> = input
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect();

    let mut map: HashMap<(u64, usize), usize> = HashMap::new();

    let mut result = 0;

    for num in input_vec.into_iter() {
        result += traverse_tree(num, 75, &mut map);
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
