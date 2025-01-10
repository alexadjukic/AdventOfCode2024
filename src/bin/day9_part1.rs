use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day9.txt").unwrap();
    let mut input = String::new();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let input_vec: Vec<u32> = input.chars().filter_map(|char| char.to_digit(10)).collect();

    let mut blocks = Vec::new();
    for (i, value) in input_vec.iter().enumerate() {
        for _ in 0..*value as i32 {
            blocks.push(if i % 2 == 0 {
                (i / 2).to_string()
            } else {
                String::from(".")
            });
        }
    }
    println!("{blocks:?}");

    let mut i = 0;
    let mut j = blocks.len() - 1;

    while i <= j {
        if blocks[i] != "." {
            i += 1;
            continue;
        }

        if blocks[j] == "." {
            j -= 1;
            continue;
        }

        let temp = blocks[i].clone();
        blocks[i] = blocks[j].clone();
        blocks[j] = temp;
    }

    blocks.iter().enumerate().fold(0, |acc, (idx, num)| {
        if let Ok(parsed_num) = num.parse::<usize>() {
            acc + idx * parsed_num
        } else {
            acc
        }
    })
}

#[test]
fn test_example() {
    let input = String::from("2333133121414131402");

    let solution = solve(input);

    assert_eq!(solution, 1928);
}
