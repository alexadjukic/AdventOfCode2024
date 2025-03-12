use std::{fs::File, io::Read, ops::BitXor};

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

fn solve(input: String) -> String {
    let mut input_iter = input.split("\n\n");

    let mut registers = input_iter
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .skip(2)
                .next()
                .unwrap()
                .parse::<usize>()
                .ok()
        })
        .collect::<Vec<_>>();

    let program = input_iter
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

    let mut output = vec![];

    let mut i = 0;
    while i < program.len() {
        let mut should_increment = true;

        match program[i] {
            0 => {
                let operand = get_combo_operand(program[i + 1], &registers);
                registers[0] = registers[0] / 2_usize.pow(operand as u32);
            }
            1 => {
                registers[1] = registers[1].bitxor(program[i + 1]);
            }
            2 => {
                let operand = get_combo_operand(program[i + 1], &registers);
                registers[1] = operand % 8;
            }
            3 => {
                if registers[0] != 0 {
                    i = program[i + 1];
                    should_increment = false;
                }
            }
            4 => {
                registers[1] = registers[1].bitxor(registers[2]);
            }
            5 => {
                output.push(get_combo_operand(program[i + 1], &registers) % 8);
            }
            6 => {
                let operand = get_combo_operand(program[i + 1], &registers);
                registers[1] = registers[0] / 2_usize.pow(operand as u32);
            }
            7 => {
                let operand = get_combo_operand(program[i + 1], &registers);
                registers[2] = registers[0] / 2_usize.pow(operand as u32);
            }
            _ => {}
        }

        if should_increment {
            i += 2;
        }
    }

    output
        .iter()
        .fold(String::new(), |acc, el| acc + &el.to_string() + ",")
        .trim_end_matches(",")
        .to_string()
}

fn get_combo_operand(operand: usize, registers: &Vec<usize>) -> usize {
    match operand {
        0..=3 => operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => 0,
    }
}

#[test]
fn test_example() {
    let input = String::from(
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    );

    let solution = solve(input);

    assert_eq!(solution, "4,6,3,5,6,3,5,2,1,0");
}
