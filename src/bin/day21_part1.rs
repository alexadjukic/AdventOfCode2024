use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day21.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let codes: Vec<&str> = input.lines().collect();
    let mut result = 0;

    for code in &codes {
        let mut dir_keys = VecDeque::new();

        let mut curr_loc = (0, 2);
        for ch in code.chars() {
            let paths = get_dir_keys(&mut curr_loc, ch);

            dir_keys.push_back(
                paths
                    .into_iter()
                    .map(|path| path + "A")
                    .collect::<Vec<String>>(),
            );
        }

        let mut dir_codes = get_combinations(&mut dir_keys);

        for _ in 0..2 {
            let mut min_len = usize::MAX;
            let mut best = VecDeque::new();
            for dir_code in dir_codes {
                let mut dir_dir_keys = VecDeque::new();

                let mut curr_loc = (1, 2);
                for ch in dir_code.chars() {
                    let paths = get_dir_dir_keys(&mut curr_loc, ch);

                    dir_dir_keys.push_back(
                        paths
                            .into_iter()
                            .map(|path| path + "A")
                            .collect::<Vec<String>>(),
                    );
                }

                let dir_dir_codes = get_combinations(&mut dir_dir_keys);

                let shortest = dir_dir_codes.iter().map(|code| code.len()).min().unwrap();

                if min_len == shortest {
                    best.extend(dir_dir_codes.clone());
                    min_len = shortest;
                } else if min_len > shortest {
                    best = dir_dir_codes.clone();
                    min_len = shortest;
                }
            }
            dir_codes = best.clone();
        }

        let parsed_num = code
            .chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let shortest_path = dir_codes.iter().map(|code| code.len()).min().unwrap();
        result += shortest_path * parsed_num;
    }

    result
}

fn get_combinations(input: &mut VecDeque<Vec<String>>) -> VecDeque<String> {
    let mut combinations = VecDeque::new();
    combinations.push_back(String::new());
    while let Some(suffix) = input.pop_front() {
        let len = combinations.len();
        for _ in 0..len {
            let prefix = combinations.pop_front().unwrap();
            for a in &suffix {
                combinations.push_back(prefix.clone() + a);
            }
        }
    }
    combinations
}

fn get_dir_keys(curr_loc: &mut (i32, i32), ch: char) -> HashSet<String> {
    let target_loc = num_to_loc(&ch);
    let diff_vert = curr_loc.0 - target_loc.0;
    let diff_hor = curr_loc.1 - target_loc.1;
    let vert_moves: String = std::iter::repeat(if diff_vert > 0 { 'v' } else { '^' })
        .take(diff_vert.abs() as usize)
        .collect();
    let hor_moves: String = std::iter::repeat(if diff_hor > 0 { '<' } else { '>' })
        .take(diff_hor.abs() as usize)
        .collect();

    let mut paths = HashSet::new();
    let mut chars = format!("{}{}", vert_moves, hor_moves).chars().collect();

    permute(&mut chars, 0, &mut paths);

    if curr_loc.0 == 0 {
        paths.retain(|path| {
            let pattern = std::iter::repeat('<')
                .take(curr_loc.1 as usize)
                .collect::<String>();
            !path.starts_with(&pattern)
        });
    }

    if curr_loc.1 == 0 {
        paths.retain(|path| {
            let pattern = std::iter::repeat('v')
                .take(curr_loc.0 as usize)
                .collect::<String>();
            !path.starts_with(&pattern)
        });
    }
    *curr_loc = target_loc;
    paths
}

fn get_dir_dir_keys(curr_loc: &mut (i32, i32), ch: char) -> HashSet<String> {
    let target_loc = dir_to_loc(&ch);
    let diff_vert = curr_loc.0 - target_loc.0;
    let diff_hor = curr_loc.1 - target_loc.1;
    let vert_moves: String = std::iter::repeat(if diff_vert > 0 { 'v' } else { '^' })
        .take(diff_vert.abs() as usize)
        .collect();
    let hor_moves: String = std::iter::repeat(if diff_hor > 0 { '<' } else { '>' })
        .take(diff_hor.abs() as usize)
        .collect();

    let mut paths = HashSet::new();
    let mut chars = format!("{}{}", vert_moves, hor_moves).chars().collect();

    permute(&mut chars, 0, &mut paths);

    if curr_loc.0 == 1 {
        paths.retain(|path| {
            let pattern = std::iter::repeat('<')
                .take(curr_loc.1 as usize)
                .collect::<String>();
            !path.starts_with(&pattern)
        });
    }

    if curr_loc.1 == 0 {
        paths.retain(|path| !path.starts_with("^"));
    }
    *curr_loc = target_loc;
    paths
}

fn permute(chars: &mut Vec<char>, start: usize, result: &mut HashSet<String>) {
    if start == chars.len() {
        result.insert(chars.iter().collect());
        return;
    }

    for i in start..chars.len() {
        chars.swap(start, i);
        permute(chars, start + 1, result);
        chars.swap(start, i);
    }
}

fn dir_to_loc(ch: &char) -> (i32, i32) {
    match ch {
        '<' => (0, 0),
        'v' => (0, 1),
        '>' => (0, 2),
        '^' => (1, 1),
        'A' => (1, 2),
        _ => (1, 0),
    }
}

fn num_to_loc(ch: &char) -> (i32, i32) {
    match ch {
        '0' => (0, 1),
        'A' => (0, 2),
        '1' => (1, 0),
        '2' => (1, 1),
        '3' => (1, 2),
        '4' => (2, 0),
        '5' => (2, 1),
        '6' => (2, 2),
        '7' => (3, 0),
        '8' => (3, 1),
        '9' => (3, 2),
        _ => (0, 0),
    }
}

#[test]
fn test_example() {
    let input = String::from(
        "029A
980A
179A
456A
379A",
    );

    let solution = solve(input);

    assert_eq!(solution, 126384);
}
