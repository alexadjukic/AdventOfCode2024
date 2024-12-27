use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    // let input = vec![
    //     vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
    //     vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
    //     vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
    //     vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
    //     vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
    //     vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
    //     vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
    //     vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
    //     vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
    //     vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    // ];
    let input = read_file();

    let times_found = find_matches(input);

    println!("Solution: {times_found}");
}

fn read_file() -> Vec<Vec<char>> {
    let mut file = File::open("inputs/day4.txt").unwrap();
    let mut input = String::new();

    let mut input_vec = Vec::new();
    if let Ok(_) = file.read_to_string(&mut input) {
        for line in input.split("\n") {
            let mut line_vec = Vec::new();
            for char in line.chars() {
                line_vec.push(char);
            }
            input_vec.push(line_vec);
        }
    }
    input_vec.pop();
    input_vec
}

fn find_matches(input: Vec<Vec<char>>) -> usize {
    let mut times_found = 0;

    let candidate_list = init_candidate_list(&input);
    let regex1 = Regex::new("XMAS").unwrap();
    let regex2 = Regex::new("SAMX").unwrap();

    for candidate in candidate_list.iter() {
        times_found += regex1.find_iter(candidate).collect::<Vec<_>>().len();
        times_found += regex2.find_iter(candidate).collect::<Vec<_>>().len();
    }
    times_found
}

fn init_candidate_list(input: &Vec<Vec<char>>) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut columns: Vec<String> = Vec::with_capacity(input.len());
    let mut main_diagonals: Vec<String> = Vec::with_capacity(input.len() - 3 + input[0].len() - 4);
    let mut alt_diagonals: Vec<String> = Vec::with_capacity(input.len() - 3 + input[0].len() - 4);

    for _ in 0..columns.capacity() {
        columns.push(String::from(""));
    }

    for _ in 0..main_diagonals.capacity() {
        main_diagonals.push(String::from(""));
        alt_diagonals.push(String::from(""));
    }

    for (i, line) in input.iter().enumerate() {
        lines.push(line.iter().collect());
        for (j, ch) in line.iter().enumerate() {
            columns[j] += &ch.to_string();

            let main_diagonal_index = i as i32 - j as i32 + input[0].len() as i32 - 4;
            if main_diagonal_index < main_diagonals.capacity() as i32 && main_diagonal_index > -1 {
                main_diagonals[main_diagonal_index as usize] += &ch.to_string();
            }

            let alt_diagonal_index = i as i32 + j as i32 - 3;
            if alt_diagonal_index < alt_diagonals.capacity() as i32 && alt_diagonal_index > -1 {
                alt_diagonals[alt_diagonal_index as usize] += &ch.to_string();
            }
        }
    }

    lines
        .into_iter()
        .chain(
            columns
                .into_iter()
                .chain(main_diagonals.into_iter().chain(alt_diagonals.into_iter())),
        )
        .collect::<Vec<String>>()
}
