use std::fs;
const OBSTRUCTION: char = '#';
const EMPTY: char = '.';

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day6_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 41 {
        println!("Example solution not valid for part1");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day6.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));

    if part2::solve(&example_input) != 6 {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

fn read_input(input: &String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> Result<(usize, usize), &'static str> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let current = grid[row][col];
            if current == EMPTY || current == OBSTRUCTION {
                continue;
            }
            return Ok((row, col));
        }
    }
    Err("No start found")
}

fn move_position(
    grid: &Vec<Vec<char>>,
    current_position: (usize, usize),
    direction: char,
) -> Result<(usize, usize), &'static str> {
    match direction {
        '^' => {
            // go up
            if current_position.0 > 0 {
                Ok((current_position.0 - 1, current_position.1))
            } else {
                Err("Out of bounds")
            }
        }
        '>' => {
            // go right
            if current_position.1 + 1 < grid[current_position.0].len() {
                Ok((current_position.0, current_position.1 + 1))
            } else {
                Err("Out of bounds")
            }
        }
        'v' => {
            // go down
            if current_position.0 + 1 < grid.len() {
                Ok((current_position.0 + 1, current_position.1))
            } else {
                Err("Out of bounds")
            }
        }
        '<' => {
            // go left
            if current_position.1 > 0 {
                Ok((current_position.0, current_position.1 - 1))
            } else {
                Err("Out of bounds")
            }
        }
        _ => Err("Invalid direction"),
    }
}

fn turn_right(direction: char) -> char {
    // turn right 90 degrees
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => ' ',
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            print!("{}", grid[row][col]);
        }
        println!();
    }
}

mod part1 {
    use crate::day6::{find_start, move_position, read_input, turn_right, OBSTRUCTION};
    use std::collections::HashSet;

    pub(crate) fn solve(input: &String) -> usize {
        let grid = read_input(input);
        let starting_position = find_start(&grid).unwrap();
        let mut current_position = starting_position;
        let mut direction = grid[starting_position.0][starting_position.1];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        'move_loop: loop {
            let mut next_position = current_position;
            let mut previous_next_position = current_position;
            while grid[next_position.0][next_position.1] != OBSTRUCTION {
                visited.insert(next_position);
                previous_next_position = next_position;
                match move_position(&grid, next_position, direction) {
                    Ok(pos) => next_position = pos,
                    Err(_) => break 'move_loop,
                }
            }

            direction = turn_right(direction);
            current_position = previous_next_position;
        }
        visited.len()
    }
}

mod part2 {
    use crate::day6::{find_start, move_position, print_grid, read_input, turn_right, OBSTRUCTION};
    use std::collections::HashSet;
    use std::thread;
    use std::thread::JoinHandle;

    pub(crate) fn solve(input: &String) -> usize {
        let grid = read_input(input);
        print_grid(&grid);
        let starting_position = find_start(&grid).unwrap();
        let mut current_position = starting_position;
        let mut direction = grid[starting_position.0][starting_position.1];
        let mut visited: Vec<(char, (usize, usize))> = Vec::new();
        'move_loop: loop {
            let mut next_position = current_position;
            let mut previous_next_position = current_position;
            while grid[next_position.0][next_position.1] != OBSTRUCTION {
                visited.push((direction, next_position));
                previous_next_position = next_position;
                match move_position(&grid, next_position, direction) {
                    Ok(pos) => next_position = pos,
                    Err(_) => break 'move_loop,
                }
            }

            direction = turn_right(direction);
            current_position = previous_next_position;
        }

        let new_obstruction_positions: HashSet<(usize, usize)> = visited
            .iter()
            .map(|(_, position)| *position)
            .filter(|position| *position != starting_position)
            .map(|position| {
                let mut new_grid = grid.clone();
                new_grid[position.0][position.1] = OBSTRUCTION;
                return thread::spawn(move || {
                    return if is_loop(&new_grid) {
                        Some(position)
                    } else {
                        None
                    };
                });
            })
            .map(JoinHandle::join)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        /**
         * Example solutions: (6,3), (7,6), (7,7), (8,1), (8,3), (9,7)
         */
        println!("Total: {}", new_obstruction_positions.len());
        new_obstruction_positions.len()
    }

    fn is_loop(grid: &Vec<Vec<char>>) -> bool {
        // print_grid(grid);
        let starting_position = find_start(&grid).unwrap();
        let mut current_position = starting_position;
        let mut direction = grid[starting_position.0][starting_position.1];
        let mut visited: HashSet<(char, (usize, usize))> = HashSet::new();
        'move_loop: loop {
            let mut next_position = current_position;
            let mut previous_next_position = current_position;
            while grid[next_position.0][next_position.1] != OBSTRUCTION {
                if !visited.insert((direction, next_position)) {
                    // we've already been here going this direction so we're in a loop!
                    return true;
                }

                previous_next_position = next_position;
                match move_position(&grid, next_position, direction) {
                    Ok(pos) => next_position = pos,
                    Err(_) => break 'move_loop,
                }
            }

            direction = turn_right(direction);
            current_position = previous_next_position;
        }
        false
    }
}
