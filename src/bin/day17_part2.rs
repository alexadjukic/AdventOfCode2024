use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day17.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let input_iter = input.split("\n\n");

    let program = input_iter
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|text| {
            text.split(",")
                .filter_map(|char| char.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    find(&program[..], 0).unwrap()
}

fn find(program: &[usize], answer: usize) -> Option<usize> {
    if program.len() == 0 {
        return Some(answer);
    }
    for mut b in 0..8 {
        let a = answer << 3 | b;
        b = b ^ 7;
        let c = a >> b;
        b = b ^ 7;
        b = b ^ c;
        if b % 8 == *program.last().unwrap() {
            if let Some(sub) = find(&program[..program.len() - 1], a) {
                return Some(sub);
            }
        }
    }
    None
}

#[test]
fn test_example() {
    let input = String::from(
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    );

    let solution = solve(input);

    assert_eq!(solution, 117440);
}
