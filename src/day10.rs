#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve() {
    let day = 10;
    let part1_example_solution = 36;

    let example_input = fs::read_to_string(format!(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day{day}_example.txt",
    ))
    .expect("Unable to read input");
    if part1::solve(&example_input) != part1_example_solution {
        println!("Example solution not valid for part1");
        return;
    }

    let input = fs::read_to_string(format!(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day{day}.txt",
    ))
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));

    let part2_example_solution = 81;

    if part2::solve(&example_input) != part2_example_solution {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

fn read_grid(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>())
                .filter_map(Result::ok)
                .collect()
        })
        .collect()
}

fn find_start_positions(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut start_positions: Vec<(usize, usize)> = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 0 {
                start_positions.push((row, col));
            }
        }
    }
    start_positions
}

fn calculate_reachable(
    grid: &Vec<Vec<i32>>,
    start_positions: &Vec<(usize, usize)>,
) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let mut reachable_from: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut positions = start_positions.clone();
    while !positions.is_empty() {
        let current_position = positions.pop().unwrap();
        let (row, col) = current_position;
        let current_value = grid[row][col];
        if current_value == 9 {
            // this is an end!
            continue;
        }

        let mut to_check: Vec<(usize, usize)> = Vec::with_capacity(4);
        match row.checked_sub(1) {
            Some(row) => to_check.push((row, col)),
            None => {}
        }
        match col.checked_sub(1) {
            Some(col) => to_check.push((row, col)),
            None => {}
        }
        if row < grid.len() - 1 {
            to_check.push((row + 1, col));
        }
        if col < grid[row].len() - 1 {
            to_check.push((row, col + 1));
        }

        let desired_value = current_value + 1;
        for (row, col) in to_check {
            if grid[row][col] == desired_value {
                positions.push((row, col));
                match reachable_from.get_mut(&current_position) {
                    Some(path) => {
                        path.insert((row, col));
                    }
                    None => {
                        reachable_from.insert(current_position, HashSet::from([(row, col)]));
                    }
                }
            }
        }
    }
    reachable_from
}

mod part1 {
    use crate::day10::{calculate_reachable, find_start_positions, read_grid};
    use std::collections::{HashMap, HashSet, VecDeque};

    pub fn solve(input: &String) -> i32 {
        let grid = read_grid(input);
        let start_positions = find_start_positions(&grid);
        let reachable_from = calculate_reachable(&grid, &start_positions);
        start_positions
            .iter()
            .map(|&position| score(position, &grid, &reachable_from))
            .sum()
    }

    fn score(
        position: (usize, usize),
        grid: &Vec<Vec<i32>>,
        reachable: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    ) -> i32 {
        let mut to_explore = VecDeque::from([position]);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut score = 0;
        while !to_explore.is_empty() {
            let next = to_explore.pop_front().unwrap();
            if visited.insert(next) {
                if grid[next.0][next.1] == 9 {
                    score += 1;
                    continue;
                }

                match reachable.get(&next) {
                    Some(set) => {
                        for &p in set {
                            to_explore.push_back(p);
                        }
                    }
                    None => {}
                }
            }
        }
        score
    }
}

mod part2 {
    use crate::day10::{calculate_reachable, find_start_positions, read_grid};
    use std::collections::{HashMap, HashSet, VecDeque};

    pub fn solve(input: &String) -> i32 {
        let grid = read_grid(input);
        let start_positions = find_start_positions(&grid);
        let reachable_from = calculate_reachable(&grid, &start_positions);

        start_positions
            .iter()
            .map(|&position| rating(position, &grid, &reachable_from))
            .sum()
    }

    fn rating(
        start_position: (usize, usize),
        grid: &Vec<Vec<i32>>,
        reachable: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    ) -> i32 {
        let mut to_explore: VecDeque<Vec<(usize, usize)>> = VecDeque::from([vec![start_position]]);
        let mut visited: HashSet<Vec<(usize, usize)>> = HashSet::new();
        while !to_explore.is_empty() {
            let next = to_explore.pop_front().unwrap();
            if visited.insert(next.clone()) {
                match next.last() {
                    Some(last) => match reachable.get(last) {
                        Some(set) => {
                            for p in set {
                                let mut path = next.clone();
                                path.push(*p);
                                to_explore.push_back(path);
                            }
                        }
                        None => {}
                    },
                    None => {}
                }
            }
        }

        visited
            .iter()
            .filter(|&v| match v.last() {
                None => false,
                Some(position) => grid[position.0][position.1] == 9,
            })
            .count() as i32
    }
}
