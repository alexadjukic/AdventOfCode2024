use std::{fs::File, io::Read};

#[derive(PartialEq)]
enum Order {
    Ascending,
    Descending,
    Undefined,
}

fn main() {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    // let reports = vec![
    //     vec![7, 6, 4, 2, 1],
    //     vec![1, 2, 7, 8, 9],
    //     vec![9, 7, 6, 2, 1],
    //     vec![1, 3, 2, 4, 5],
    //     vec![8, 6, 4, 4, 1],
    //     vec![1, 3, 6, 7, 9],
    // ];

    let mut file = File::open("input.txt").unwrap();

    let mut input = String::new();
    if let Ok(_) = file.read_to_string(&mut input) {
        for line in input.split("\n") {
            let mut temp_vec: Vec<i32> = Vec::new();
            for string_number in line.split_whitespace() {
                let number = match string_number.parse::<i32>() {
                    Ok(number) => number,
                    Err(_) => break,
                };
                temp_vec.push(number);
            }
            reports.push(temp_vec);
        }
        reports.pop();

        let mut safe_reports = 0;
        for report in reports.iter() {
            safe_reports += 1;
            let order: Order;
            match report[0] - report[1] {
                1..=3 => order = Order::Descending,
                -3..=-1 => order = Order::Ascending,
                _ => order = Order::Undefined,
            }
            for i in 1..report.len() - 1 {
                match report[i] - report[i + 1] {
                    1..=3 if order == Order::Descending => continue,
                    -3..=-1 if order == Order::Ascending => continue,
                    _ => {
                        safe_reports -= 1;
                        break;
                    }
                }
            }
        }
        println!("Solution: {safe_reports}");
    }
}
