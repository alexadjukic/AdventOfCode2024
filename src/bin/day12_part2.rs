use grid::*;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("inputs/day12.txt").unwrap();
    let mut input = String::new();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let solution = solve(input);

    println!("Solution: {solution}");
}

fn solve(input: String) -> usize {
    let garden = Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|ch| !ch.is_whitespace())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
    );

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (coords, _) in garden.indexed_iter() {
        if !visited.contains(&coords) {
            regions.push(find_region(coords, None, &garden, &mut visited));
        }
    }

    let mut price = 0;

    for region in regions {
        let mut top_edges: HashSet<(i32, i32)> = HashSet::new();
        let mut bot_edges: HashSet<(i32, i32)> = HashSet::new();
        let mut left_edges: HashSet<(i32, i32)> = HashSet::new();
        let mut right_edges: HashSet<(i32, i32)> = HashSet::new();
        for coordinate in region.iter() {
            let top_edge = (coordinate.0 as i32 * 2 - 1, coordinate.1 as i32 * 2);
            let bot_edge = (coordinate.0 as i32 * 2 + 1, coordinate.1 as i32 * 2);
            let left_edge = (coordinate.0 as i32 * 2, coordinate.1 as i32 * 2 - 1);
            let right_edge = (coordinate.0 as i32 * 2, coordinate.1 as i32 * 2 + 1);

            if !bot_edges.remove(&top_edge) {
                top_edges.insert(top_edge);
            }

            if !top_edges.remove(&bot_edge) {
                bot_edges.insert(bot_edge);
            }

            if !left_edges.remove(&right_edge) {
                right_edges.insert(right_edge);
            }

            if !right_edges.remove(&left_edge) {
                left_edges.insert(left_edge);
            }
        }
        let mut edge_num = count_vertical_edges(top_edges);
        edge_num += count_vertical_edges(bot_edges);
        edge_num += count_horizontal_edges(left_edges);
        edge_num += count_horizontal_edges(right_edges);

        price += edge_num * region.len();
    }

    price
}

fn count_vertical_edges(edges: HashSet<(i32, i32)>) -> usize {
    let mut edge_num = 0;

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for coords in edges {
        map.entry(coords.0)
            .and_modify(|v| v.push(coords.1))
            .or_insert(vec![coords.1]);
    }

    for entry in map.values_mut() {
        edge_num += 1;
        entry.sort();
        let mut last_item = entry.first().unwrap();
        for item in entry.iter().skip(1) {
            if *item != last_item + 2 {
                edge_num += 1;
            }
            last_item = item;
        }
    }
    edge_num
}

fn count_horizontal_edges(edges: HashSet<(i32, i32)>) -> usize {
    let mut edge_num = 0;

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for coords in edges {
        map.entry(coords.1)
            .and_modify(|v| v.push(coords.0))
            .or_insert(vec![coords.0]);
    }

    for entry in map.values_mut() {
        edge_num += 1;
        entry.sort();
        let mut last_item = entry.first().unwrap();
        for item in entry.iter().skip(1) {
            if *item != last_item + 2 {
                edge_num += 1;
            }
            last_item = item;
        }
    }
    edge_num
}

fn find_region(
    coords: (usize, usize),
    symbol: Option<char>,
    garden: &Grid<char>,
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut region = HashSet::new();

    let symbol = symbol.unwrap_or(*garden.get(coords.0, coords.1).unwrap());

    if symbol != *garden.get(coords.0, coords.1).unwrap() {
        return region;
    }

    region.insert(coords);
    visited.insert(coords);

    let left = if coords.1 == 0 {
        None
    } else {
        Some((coords.0, coords.1 - 1))
    };
    let right = if coords.1 + 1 == garden.size().1 {
        None
    } else {
        Some((coords.0, coords.1 + 1))
    };
    let down = if coords.0 + 1 == garden.size().0 {
        None
    } else {
        Some((coords.0 + 1, coords.1))
    };
    let up = if coords.0 == 0 {
        None
    } else {
        Some((coords.0 - 1, coords.1))
    };

    if let Some(next_coords) = left {
        if !visited.contains(&next_coords) {
            region.extend(find_region(next_coords, Some(symbol), garden, visited).iter());
        }
    }

    if let Some(next_coords) = right {
        if !visited.contains(&next_coords) {
            region.extend(find_region(next_coords, Some(symbol), garden, visited).iter());
        }
    }

    if let Some(next_coords) = down {
        if !visited.contains(&next_coords) {
            region.extend(find_region(next_coords, Some(symbol), garden, visited).iter());
        }
    }

    if let Some(next_coords) = up {
        if !visited.contains(&next_coords) {
            region.extend(find_region(next_coords, Some(symbol), garden, visited).iter());
        }
    }

    region
}

#[test]
fn test_example() {
    let input = String::from(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    );

    let solution = solve(input);

    assert_eq!(solution, 1206);
}
