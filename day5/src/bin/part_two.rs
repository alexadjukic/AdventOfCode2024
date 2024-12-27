use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    //     let mut input = String::from(
    //         "47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13

    // 75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47",
    //     );

    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    if let Ok(_) = file.read_to_string(&mut input) {
        let (rules, mut updates) = parse_input(input);

        let correct_updates = reorder_and_sum(&rules, &mut updates);

        println!("Solution: {correct_updates:?}");
    }
}

fn reorder_and_sum(rules: &HashMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for update in updates.iter_mut() {
        let mut corrected = false;
        for i in 0..update.len() {
            for j in i..update.len() {
                if let Some(rule) = rules.get(&update[i]) {
                    if rule.contains(&update[j]) {
                        if i > j {}
                        let temp = update[i];
                        update[i] = update[j];
                        update[j] = temp;
                        corrected = true;
                    }
                }
            }
        }
        if corrected {
            sum += update.get(update.len() / 2).unwrap();
        }
    }
    sum
}

fn parse_input(input: String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut is_rule = true;
    for line in input.split("\n") {
        if line.is_empty() {
            is_rule = false;
            continue;
        }

        if is_rule {
            let rule: Vec<_> = line.split("|").collect();
            let before = rule[0].parse::<i32>().unwrap();
            let after = rule[1].parse::<i32>().unwrap();
            rules
                .entry(after)
                .and_modify(|e| e.push(before))
                .or_insert(vec![before]);
        } else {
            let mut update: Vec<i32> = Vec::new();
            for str_num in line.split(",") {
                update.push(str_num.parse::<i32>().unwrap());
            }
            updates.push(update);
        }
    }
    (rules, updates)
}
