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
        if check(&towels, pattern, &mut HashMap::new()) {
            result += 1;
        }
    }

    result
}

fn check(towels: &Vec<&str>, pattern: &str, cache: &mut HashMap<String, bool>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if let Some(result) = cache.get(pattern) {
        return *result;
    }

    let longest_towel = towels.iter().map(|towel| towel.len()).max().unwrap();
    for i in (1..=longest_towel.min(pattern.len())).rev() {
        if towels.contains(&&pattern[..i]) && check(towels, &pattern[i..], cache) {
            cache.insert(pattern.to_string(), true);
            return true;
        }
    }

    cache.insert(pattern.to_string(), false);
    false
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
