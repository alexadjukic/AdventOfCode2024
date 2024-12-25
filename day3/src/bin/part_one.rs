use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    if let Ok(_) = file.read_to_string(&mut input) {
        let regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").unwrap();
        let result = regex.captures_iter(&input).fold(0, |acc, m| {
            acc + &m[1].parse::<i32>().unwrap() * &m[2].parse::<i32>().unwrap()
        });
        println!("Solution: {result}");
    }
}
