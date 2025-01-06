use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn main() {
    let mut file = File::open("inputs/day8.txt").unwrap();
    let mut input = String::new();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let now = Instant::now();

    let solution = solve_seq(input.clone());

    println!("Solution sequential: {solution}, time: {:?}", now.elapsed());

    let now = Instant::now();

    let solution = solve_par(input);

    println!("Solution parallel: {solution}, time: {:?}", now.elapsed());
}

fn solve_par(input: String) -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let col_num = input.lines().next().unwrap().chars().count();
    let row_num = input.lines().collect::<String>().chars().count() / col_num;

    for (i, char) in input.lines().collect::<String>().chars().enumerate() {
        if char == '.' || char == '\n' {
            continue;
        }

        let col = (i % col_num) as i32;
        let row = (i / col_num) as i32;

        antennas
            .entry(char)
            .and_modify(|e| e.push((row, col)))
            .or_insert(vec![(row, col)]);
    }

    let antinodes = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = Vec::new();
    for (_key, value) in antennas.into_iter() {
        let set_cpy = Arc::clone(&antinodes);
        let handle = thread::spawn(move || {
            for i in 0..value.len() {
                for j in (i + 1)..value.len() {
                    let antenna1 = value[i];
                    let antenna2 = value[j];

                    let diff_0 = antenna1.0 - antenna2.0;
                    let diff_1 = antenna1.1 - antenna2.1;

                    let new_0 = antenna1.0 + diff_0;
                    let new_1 = antenna1.1 + diff_1;

                    if new_0 < row_num as i32 && new_1 < col_num as i32 && new_0 >= 0 && new_1 >= 0
                    {
                        let mut value = set_cpy.lock().unwrap();
                        value.insert((new_0, new_1));
                    }

                    let new_0 = antenna2.0 - diff_0;
                    let new_1 = antenna2.1 - diff_1;

                    if new_0 < row_num as i32 && new_1 < col_num as i32 && new_0 >= 0 && new_1 >= 0
                    {
                        let mut value = set_cpy.lock().unwrap();
                        value.insert((new_0, new_1));
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    return antinodes.lock().unwrap().len();
}

fn solve_seq(input: String) -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let col_num = input.lines().next().unwrap().chars().count();
    let row_num = input.lines().collect::<String>().chars().count() / col_num;

    for (i, char) in input.lines().collect::<String>().chars().enumerate() {
        if char == '.' || char == '\n' {
            continue;
        }

        let col = (i % col_num) as i32;
        let row = (i / col_num) as i32;

        antennas
            .entry(char)
            .and_modify(|e| e.push((row, col)))
            .or_insert(vec![(row, col)]);
    }

    let mut antinodes = HashSet::new();
    for (_key, value) in antennas.iter() {
        for i in 0..value.len() {
            for j in (i + 1)..value.len() {
                let antenna1 = value[i];
                let antenna2 = value[j];

                let diff_0 = antenna1.0 - antenna2.0;
                let diff_1 = antenna1.1 - antenna2.1;

                let new_0 = antenna1.0 + diff_0;
                let new_1 = antenna1.1 + diff_1;

                if new_0 < row_num as i32 && new_1 < col_num as i32 && new_0 >= 0 && new_1 >= 0 {
                    antinodes.insert((new_0, new_1));
                }

                let new_0 = antenna2.0 - diff_0;
                let new_1 = antenna2.1 - diff_1;

                if new_0 < row_num as i32 && new_1 < col_num as i32 && new_0 >= 0 && new_1 >= 0 {
                    antinodes.insert((new_0, new_1));
                }
            }
        }
    }
    antinodes.len()
}

#[test]
fn test_example() {
    let input = String::from(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    );

    let solution = solve_seq(input);

    assert_eq!(solution, 14);
}
