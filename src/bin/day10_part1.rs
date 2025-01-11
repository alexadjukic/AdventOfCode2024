use std::{collections::HashSet, fs::File, io::Read, time::Instant};

use grid::Grid;

fn main() {
    let mut file = File::open("inputs/day10.txt").unwrap();
    let mut input = String::new();

    if let Err(e) = file.read_to_string(&mut input) {
        eprintln!("{e}");
        return;
    }

    let now = Instant::now();
    let solution = solve(input);

    println!("Solution: {solution}, time: {:?}", now.elapsed());
}

fn solve(input: String) -> usize {
    let grid: Grid<u32> = Grid::from(
        input
            .lines()
            .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
            .collect::<Vec<_>>(),
    );

    let mut trailheads: HashSet<(usize, usize)> = HashSet::new();
    let mut total_score = 0;

    for (coordinate, item) in grid.indexed_iter() {
        if *item == 0 {
            trailheads.insert(coordinate);
        }
    }

    for trailhead in trailheads.into_iter() {
        let mut score = HashSet::new();
        rank_trailhead(trailhead, &grid, &mut score);
        total_score += score.len();
    }

    total_score
}

fn rank_trailhead(
    trailhead: (usize, usize),
    grid: &Grid<u32>,
    score: &mut HashSet<(usize, usize)>,
) {
    let current_value = grid[trailhead];

    if current_value == 9 {
        score.insert(trailhead);
    }

    if let Some(next_pos) = move_left(trailhead) {
        let next_value = grid[next_pos];
        if current_value + 1 == next_value {
            rank_trailhead(next_pos, grid, score);
        }
    }

    if let Some(next_pos) = move_right(trailhead, grid.size().1) {
        let next_value = grid[next_pos];
        if current_value + 1 == next_value {
            rank_trailhead(next_pos, grid, score);
        }
    }

    if let Some(next_pos) = move_up(trailhead) {
        let next_value = grid[next_pos];
        if current_value + 1 == next_value {
            rank_trailhead(next_pos, grid, score);
        }
    }

    if let Some(next_pos) = move_down(trailhead, grid.size().0) {
        let next_value = grid[next_pos];
        if current_value + 1 == next_value {
            rank_trailhead(next_pos, grid, score);
        }
    }
}

fn move_left(position: (usize, usize)) -> Option<(usize, usize)> {
    if position.1 == 0 {
        None
    } else {
        Some((position.0, position.1 - 1))
    }
}

fn move_right(position: (usize, usize), max: usize) -> Option<(usize, usize)> {
    if position.1 + 1 == max {
        None
    } else {
        Some((position.0, position.1 + 1))
    }
}

fn move_down(position: (usize, usize), max: usize) -> Option<(usize, usize)> {
    if position.0 + 1 == max {
        None
    } else {
        Some((position.0 + 1, position.1))
    }
}

fn move_up(position: (usize, usize)) -> Option<(usize, usize)> {
    if position.0 == 0 {
        None
    } else {
        Some((position.0 - 1, position.1))
    }
}

#[test]
fn test_example() {
    let input = String::from(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );

    let solution = solve(input);

    assert_eq!(solution, 36);
}
