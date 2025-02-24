use grid::*;
use std::{collections::HashSet, fs::File, io::Read};

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
    let mut next_regions: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    next_regions.push((0, 0));
    for (coords, _) in garden.indexed_iter() {
        if !visited.contains(&coords) {
            regions.push(find_region(coords, None, &garden, &mut visited));
        }
    }

    let mut price = 0;
    for region in regions {
        let mut perimeter = 0;
        for coords in region.iter() {
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

            match left {
                Some(value) => {
                    if !region.contains(&value) {
                        perimeter += 1;
                    }
                }
                None => perimeter += 1,
            }

            match right {
                Some(value) => {
                    if !region.contains(&value) {
                        perimeter += 1;
                    }
                }
                None => perimeter += 1,
            }

            match down {
                Some(value) => {
                    if !region.contains(&value) {
                        perimeter += 1;
                    }
                }
                None => perimeter += 1,
            }

            match up {
                Some(value) => {
                    if !region.contains(&value) {
                        perimeter += 1;
                    }
                }
                None => perimeter += 1,
            }
        }
        price += region.len() * perimeter;
    }

    price
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

    assert_eq!(solution, 1930);
}
