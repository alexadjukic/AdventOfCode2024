use std::{fs::File, io::Read};

fn main() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    let info = f.read_to_string(&mut input);
    if let Ok(_) = info {
        for line in input.split("\n") {
            for string_number in line.split("   ") {
                if let Ok(number) = string_number.parse::<i32>() {
                    if list1.len() == list2.len() {
                        list1.push(number);
                    } else {
                        list2.push(number);
                    }
                }
            }
        }
        list1.sort();
        list2.sort();

        let mut total_distance = 0;
        for i in 0..list1.len() {
            total_distance += i32::abs(list1[i] - list2[i]);
        }
        println!("Solution: {total_distance}");
    }
}
