use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day22.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let numbers: Vec<usize> = input.lines().filter_map(|line| line.parse().ok()).collect();

    let mut result = 0;

    for number in numbers {
        let mut x = number;
        for _ in 0..2000 {
            x = (x ^ (x << 6)) & 16777215;
            x = (x ^ (x >> 5)) & 16777215;
            x = (x ^ (x << 11)) & 16777215;
        }

        result += x;
    }

    result
}

#[test]
fn test_example() {
    let input = String::from(
        "1
10
100
2024",
    );

    let solution = solve(input);

    assert_eq!(solution, 37327623);
}
