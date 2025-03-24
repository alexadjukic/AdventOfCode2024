use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day23.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> String {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let computers: Vec<String> = line.split("-").map(|part| part.to_string()).collect();

        map.entry(computers[0].clone())
            .and_modify(|entry| {
                entry.insert(computers[1].clone());
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(computers[1].clone());
                set
            });

        map.entry(computers[1].clone())
            .and_modify(|entry| {
                entry.insert(computers[0].clone());
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(computers[0].clone());
                set
            });
    });

    let mut sets = Vec::new();

    for (key, _) in map.iter() {
        let mut set = HashSet::new();
        search(key.to_string(), &map, &mut set);
        sets.push(set);
    }

    let mut biggest_party = sets
        .into_iter()
        .max_by_key(|set| set.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<String>>();

    biggest_party.sort();

    biggest_party.join(",")
}

fn search(
    computer: String,
    map: &HashMap<String, HashSet<String>>,
    required_set: &mut HashSet<String>,
) {
    let neighbours = map.get(&computer).unwrap();
    if required_set.difference(neighbours).count() == 0 {
        for neighbour in neighbours.difference(&required_set.clone()) {
            required_set.insert(computer.clone());
            search(neighbour.to_string(), map, required_set);
        }
    }
}

#[test]
fn test_example() {
    let input = String::from(
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    );

    let solution = solve(input);

    assert_eq!(solution, "co,de,ka,ta");
}
