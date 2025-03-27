use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

fn main() {
    let mut input = String::new();
    let mut file = File::open("inputs/day24.txt").unwrap();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let mut input_iter = input.split("\n\n");

    let mut values = HashMap::new();

    values.extend(input_iter.next().unwrap().lines().map(|line| {
        let mut line_iter = line.split(": ");

        let variable = line_iter.next().unwrap();
        let value = line_iter.next().unwrap().parse::<usize>().unwrap();

        (variable, value)
    }));

    let mut z_wires = usize::MIN;

    let mut gates = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_iter = line.split_whitespace();

            let operand1 = line_iter.next().unwrap();
            let operation = line_iter.next().unwrap();
            let operand2 = line_iter.next().unwrap();
            line_iter.next();
            let result = line_iter.next().unwrap();

            if result.starts_with("z") {
                let num: usize = result[1..].parse().unwrap();
                if num > z_wires {
                    z_wires = num;
                }
            }

            (operand1, operation, operand2, result)
        })
        .collect::<VecDeque<_>>();

    while let Some(gate) = gates.pop_front() {
        if let (Some(operand1), Some(operand2)) = (values.get(gate.0), values.get(gate.2)) {
            if gate.1 == "AND" {
                values.insert(gate.3, operand1 & operand2);
            } else if gate.1 == "OR" {
                values.insert(gate.3, operand1 | operand2);
            } else {
                values.insert(gate.3, operand1 ^ operand2);
            }
        } else {
            gates.push_back(gate);
        }
    }

    let mut result = 0;

    for i in (0..z_wires + 1).rev() {
        let register = if i < 10 {
            format!("z0{i}")
        } else {
            format!("z{i}")
        };

        result <<= 1;
        result ^= values.get(&register[..]).unwrap();
    }

    result
}

#[test]
fn test_example() {
    let input = String::from(
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
    );

    let solution = solve(input);

    assert_eq!(solution, 2024);
}
