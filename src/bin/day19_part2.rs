use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day19.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let mut input_iter = input.split("\n\n");
    let towels: Vec<&str> = input_iter.next().unwrap().split(", ").collect();

    let patterns: Vec<&str> = input_iter.next().unwrap().lines().collect();

    let mut result = 0;

    for pattern in &patterns {
        result += check(&towels, pattern, &mut HashMap::new());
    }

    result
}

fn check(towels: &Vec<&str>, pattern: &str, cache: &mut HashMap<String, usize>) -> usize {
    if pattern.len() == 0 {
        return 1;
    }

    if let Some(result) = cache.get(pattern) {
        return *result;
    }

    let mut result = 0;

    let longest_towel = towels.iter().map(|towel| towel.len()).max().unwrap();
    for i in (1..=longest_towel.min(pattern.len())).rev() {
        if towels.contains(&&pattern[..i]) {
            let x = check(towels, &pattern[i..], cache);
            result += x;
        }
    }

    cache.insert(pattern.to_string(), result);
    result
}

#[test]
fn test_example() {
    let input = String::from(
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    );

    let solution = solve(input);

    assert_eq!(solution, 1);
}
