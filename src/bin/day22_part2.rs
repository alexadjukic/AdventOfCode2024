use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day22.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> i32 {
    let numbers: Vec<usize> = input.lines().filter_map(|line| line.parse().ok()).collect();

    let mut map: HashMap<VecDeque<i32>, i32> = HashMap::new();
    for number in numbers {
        let mut x = number;
        let mut prev_price = number as i32 % 10;
        let mut changes = VecDeque::with_capacity(4);
        let mut local_map: HashMap<VecDeque<i32>, i32> = HashMap::new();
        for _ in 0..4 {
            generate(&mut x);

            let price = x as i32 % 10;

            changes.push_back(x as i32 % 10 - prev_price);

            prev_price = price;
        }

        local_map.entry(changes.clone()).or_insert(prev_price);

        for _ in 4..2000 {
            generate(&mut x);

            let price = x as i32 % 10;

            changes.push_back(price - prev_price);
            changes.pop_front();

            local_map.entry(changes.clone()).or_insert(price);

            prev_price = price;
        }

        for (key, value) in local_map.into_iter() {
            map.entry(key)
                .and_modify(|entry| *entry += value)
                .or_insert(value);
        }
    }

    *map.values().max().unwrap()
}

fn generate(x: &mut usize) {
    *x = (*x ^ (*x << 6)) & 16777215;
    *x = (*x ^ (*x >> 5)) & 16777215;
    *x = (*x ^ (*x << 11)) & 16777215;
}

#[test]
fn test_example() {
    let input = String::from(
        "1
2
3
2024",
    );

    let solution = solve(input);

    assert_eq!(solution, 23);
}
