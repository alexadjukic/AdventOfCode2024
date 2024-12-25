use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").unwrap();

    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();

    if let Ok(_) = file.read_to_string(&mut input) {
        let mut result = 0;

        for slice in input.split("do()") {
            let mut iter = slice.split("don't()");
            let enabled_slice = iter.next().unwrap();
            result += regex.captures_iter(&enabled_slice).fold(0, |acc, m| {
                acc + &m[1].parse::<i32>().unwrap() * &m[2].parse::<i32>().unwrap()
            });
        }

        println!("Solution: {result}")
    }
}
