use std::{
    collections::{HashMap, HashSet},
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

    let mut complexities = 0;
    for code in codes {
        let mut dir_key_segments = vec![];
        let mut start_pos = (0, 2);
        for num_key in code.chars() {
            dir_key_segments.push(num_key_to_dir_keys(&mut start_pos, num_key));
        }

        let mut memo = HashMap::new();
        let mut total = 0;
        for segment in dir_key_segments {
            let mut shortest = usize::MAX;
            for variant in segment {
                let mut result = 0;
                let chars = variant.chars().collect::<Vec<char>>();
                result += get_move_num(&'A', &chars[0], 24, &mut memo);
                for i in 0..chars.len() - 1 {
                    result += get_move_num(&chars[i], &chars[i + 1], 24, &mut memo);
                }

                if shortest > result {
                    shortest = result;
                }
            }
            total += shortest;
        }

        let parsed_code = code
            .chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        complexities += total * parsed_code;
    }

    complexities
}

fn get_move_num(
    start: &char,
    end: &char,
    depth: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(a) = memo.get(&(*start, *end, depth)) {
        return *a;
    }

    let mut smallest = usize::MAX;
    for path in get_dir_key_path(start, end) {
        if depth == 0 {
            memo.insert((*start, *end, depth), path.len());
            return path.len();
        }

        let mut result = 0;
        result += get_move_num(&'A', &path[0], depth - 1, memo);

        for i in 0..path.len() - 1 {
            result += get_move_num(&path[i], &path[i + 1], depth - 1, memo);
        }

        if smallest > result {
            smallest = result;
        }
    }

    memo.insert((*start, *end, depth), smallest);
    smallest
}

fn num_key_to_dir_keys(start_pos: &mut (i32, i32), num_key: char) -> Vec<String> {
    let target_pos = num_to_pos(&num_key);
    let vert_diff = start_pos.0 - target_pos.0;
    let hor_diff = start_pos.1 - target_pos.1;

    let mut required_inputs = vec![];

    required_inputs.extend(std::iter::repeat_n(
        if vert_diff < 0 { '^' } else { 'v' },
        vert_diff.abs() as usize,
    ));
    required_inputs.extend(std::iter::repeat_n(
        if hor_diff < 0 { '>' } else { '<' },
        hor_diff.abs() as usize,
    ));

    let mut possible_paths = HashSet::new();

    permute(&mut required_inputs, 0, &mut possible_paths);

    if start_pos.0 == 0 {
        possible_paths.retain(|path| {
            let pattern = std::iter::repeat_n("<", start_pos.1.abs() as usize).collect::<String>();

            !path.starts_with(&pattern)
        });
    }

    if start_pos.1 == 0 {
        possible_paths.retain(|path| {
            let pattern = std::iter::repeat_n("v", start_pos.0.abs() as usize).collect::<String>();

            !path.starts_with(&pattern)
        });
    }

    *start_pos = target_pos;

    possible_paths.into_iter().map(|path| path + "A").collect()
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

fn get_dir_key_path(start: &char, end: &char) -> Vec<Vec<char>> {
    match start {
        '<' => match end {
            '<' => vec![vec!['A']],
            'v' => vec![vec!['>', 'A']],
            '>' => vec![vec!['>', '>', 'A']],
            '^' => vec![vec!['>', '^', 'A']],
            'A' => vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']],
            _ => panic!(),
        },
        'v' => match end {
            '<' => vec![vec!['<', 'A']],
            'v' => vec![vec!['A']],
            '>' => vec![vec!['>', 'A']],
            '^' => vec![vec!['^', 'A']],
            'A' => vec![vec!['>', '^', 'A'], vec!['^', '>', 'A']],
            _ => panic!(),
        },
        '>' => match end {
            '<' => vec![vec!['<', '<', 'A']],
            'v' => vec![vec!['<', 'A']],
            '>' => vec![vec!['A']],
            '^' => vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']],
            'A' => vec![vec!['^', 'A']],
            _ => panic!(),
        },
        '^' => match end {
            '<' => vec![vec!['v', '<', 'A']],
            'v' => vec![vec!['v', 'A']],
            '>' => vec![vec!['v', '>', 'A'], vec!['>', 'v', 'A']],
            '^' => vec![vec!['A']],
            'A' => vec![vec!['>', 'A']],
            _ => panic!(),
        },
        'A' => match end {
            '<' => vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']],
            'v' => vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']],
            '>' => vec![vec!['v', 'A']],
            '^' => vec![vec!['<', 'A']],
            'A' => vec![vec!['A']],
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn num_to_pos(ch: &char) -> (i32, i32) {
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
