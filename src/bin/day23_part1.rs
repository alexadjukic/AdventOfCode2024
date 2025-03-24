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

fn solve(input: String) -> usize {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let computers: Vec<&str> = line.split("-").collect();

        map.entry(computers[0])
            .and_modify(|entry| {
                entry.insert(computers[1]);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(computers[1]);
                set
            });

        map.entry(computers[1])
            .and_modify(|entry| {
                entry.insert(computers[0]);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(computers[0]);
                set
            });
    });

    let mut sets = HashSet::new();

    for (comp1, val) in map.iter() {
        if comp1.starts_with("t") {
            for comp2 in val.iter() {
                let peers_peers = map.get(comp2).unwrap();
                for comp3 in peers_peers.intersection(val) {
                    insert_sets(comp1, comp2, comp3, &mut sets);
                }
            }
        }
    }

    sets.len()
}

fn insert_sets(
    comp1: &str,
    comp2: &str,
    comp3: &str,
    seen: &mut HashSet<(String, String, String)>,
) -> bool {
    let mut a = vec![comp1, comp2, comp3];

    a.sort();

    seen.insert((a[0].to_string(), a[1].to_string(), a[2].to_string()))
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

    assert_eq!(solution, 7);
}
