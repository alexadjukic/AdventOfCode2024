use std::{fs::File, io::Read};

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
    let mut file = File::open("input.txt").unwrap();
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
    let mut matches_found = 0;
    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            if input[i][j] == 'A' {
                let top_left = input[i - 1][j - 1];
                let top_right = input[i - 1][j + 1];
                let bottom_left = input[i + 1][j - 1];
                let bottom_right = input[i + 1][j + 1];

                if top_left == 'A'
                    || top_left == 'X'
                    || top_right == 'A'
                    || top_right == 'X'
                    || bottom_left == 'A'
                    || bottom_left == 'X'
                    || bottom_right == 'A'
                    || bottom_right == 'X'
                    || top_left == bottom_right
                    || top_right == bottom_left
                {
                    continue;
                }
                matches_found += 1;
            }
        }
    }
    return matches_found;
}
