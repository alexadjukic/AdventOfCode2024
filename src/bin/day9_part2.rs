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
    let mut input_vec: Vec<u32> = input.chars().filter_map(|char| char.to_digit(10)).collect();

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

    let block_index_max = if input_vec.len() % 2 == 0 {
        input_vec.len() - 2
    } else {
        input_vec.len() - 1
    };

    let mut block_index = block_index_max;
    loop {
        let block_size = input_vec[block_index];

        for i in (1..block_index).step_by(2) {
            let free_space_size = input_vec[i];

            if free_space_size >= block_size {
                let mut start_index: usize = input_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| (idx < i).then_some(val))
                    .sum::<u32>() as usize;
                let mut end_index: usize = input_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| (idx < block_index).then_some(val))
                    .sum::<u32>() as usize
                    + block_size as usize
                    - 1;

                for _ in 0..input_vec[block_index].min(input_vec[i]) {
                    let temp = blocks[start_index].clone();
                    blocks[start_index] = blocks[end_index].clone();
                    blocks[end_index] = temp;

                    start_index += 1;
                    end_index -= 1;
                }

                if block_index == block_index_max {
                    input_vec[block_index - 1] += block_size;
                    input_vec.pop();
                } else {
                    let new_free_space =
                        input_vec[block_index - 1] + input_vec[block_index + 1] + block_size;
                    input_vec.remove(block_index);
                    input_vec.remove(block_index);
                    input_vec[block_index - 1] = new_free_space;
                }
                block_index += 2;

                if free_space_size > block_size {
                    input_vec[i] -= block_size;
                    input_vec.insert(i, block_size);
                    input_vec.insert(i, 0);
                } else if free_space_size == block_size {
                    input_vec.insert(i + 1, 0);
                    input_vec.insert(i, 0);
                }

                break;
            }
        }
        if block_index < 2 {
            break;
        } else {
            block_index -= 2;
        }
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

    assert_eq!(solution, 2858);
}
