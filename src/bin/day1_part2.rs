use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut list = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut f = File::open("inputs/day1.txt").unwrap();
    let mut input = String::new();
    let info = f.read_to_string(&mut input);
    if let Ok(_) = info {
        for line in input.split("\n") {
            let mut odd_index = true;
            for string_number in line.split("   ") {
                if let Ok(number) = string_number.parse::<i32>() {
                    if odd_index {
                        list.push(number);
                    } else {
                        *map.entry(number).or_insert(0) += 1;
                    }
                }
                odd_index = !odd_index;
            }
        }

        let mut total_distance = 0;
        for num in list.iter() {
            total_distance += num * *map.entry(*num).or_insert(0);
        }
        println!("Solution: {total_distance}");
    }
}
