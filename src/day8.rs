#![allow(dead_code)]
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day8_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 14 {
        println!("Example solution not valid for part1");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day8.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));

    if part2::solve(&example_input) != 34 {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

fn read_input(input: &String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_antennas(grid: &Vec<Vec<char>>) -> Vec<Point> {
    let mut antenna_points: Vec<Point> = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != '.' && grid[row][col] != '#' {
                antenna_points.push(Point { row, col });
            }
        }
    }
    antenna_points
}

fn group_points(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
    let antenna_points = find_antennas(&grid);
    let mut repeated_points: HashMap<char, Vec<Point>> = Default::default();
    for point in antenna_points {
        let c = grid[point.row][point.col];
        match repeated_points.get_mut(&c) {
            Some(points) => {
                points.push(point);
                points.sort();
            }
            None => {
                repeated_points.insert(c, vec![point]);
            }
        }
    }
    repeated_points
}

#[allow(dead_code)]
mod part1 {
    use crate::day8::{group_points, read_input, Point};
    use std::collections::HashSet;

    pub fn solve(input: &String) -> i32 {
        let grid = read_input(input);
        let repeated_points = group_points(&grid);
        let all: HashSet<Point> = repeated_points
            .iter()
            .filter(|(_, points)| points.len() > 1)
            .flat_map(|(_, points)| {
                let mut results: Vec<Point> = vec![];
                for i in 0..points.len() {
                    for j in (i + 1)..points.len() {
                        let point1 = points[i];
                        let point2 = points[j];
                        if point1 != point2 {
                            let antinodes = find_antinodes_for_pair(&grid, &point1, &point2);
                            for point in antinodes {
                                results.push(point);
                            }
                        }
                    }
                }
                results
            })
            .collect();
        all.len() as i32
    }

    fn find_antinodes_for_pair(
        grid: &Vec<Vec<char>>,
        point1: &Point,
        point2: &Point,
    ) -> Vec<Point> {
        if point1.row > point2.row {
            return find_antinodes_for_pair(grid, point1, point2);
        }
        // point 1 is above point 2

        let row_range = 0.0..grid.len() as f64;
        let col_range = 0.0..(grid[0].len() as f64);
        let row_distance = (point2.row as f64 - point1.row as f64).abs();
        let col_distance = (point2.col as f64 - point1.col as f64).abs();

        let mut result = vec![];

        if point1.col < point2.col {
            // point 2 is to the right of point 1, so the line is \
            let row = point1.row as f64 - row_distance;
            let col = point1.col as f64 - col_distance;
            if row_range.contains(&row) && col_range.contains(&col) {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            }
            let row = point2.row as f64 + row_distance;
            let col = point2.col as f64 + col_distance;
            if row_range.contains(&row) && col_range.contains(&col) {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            }
        } else {
            // point 2 is to the left of point 1, so the line is /
            let row = point1.row as f64 - row_distance;
            let col = point1.col as f64 + col_distance;
            if row_range.contains(&row) && col_range.contains(&col) {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            }
            let row = point2.row as f64 + row_distance;
            let col = point2.col as f64 - col_distance;
            if row_range.contains(&row) && col_range.contains(&col) {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            }
        }

        result
    }
}

mod part2 {
    use crate::day8::{group_points, read_input, Point};
    use fraction::{Fraction, Signed};
    use std::collections::HashSet;
    use std::ops::Neg;

    pub(crate) fn solve(input: &String) -> i32 {
        let grid = read_input(input);
        let repeated_points = group_points(&grid);
        let all: HashSet<Point> = repeated_points
            .iter()
            .filter(|(_, points)| points.len() > 1)
            .flat_map(|(antenna, points)| {
                println!("Antenna: {:?}", antenna);
                let mut results: Vec<Point> = vec![];
                for i in 0..points.len() {
                    for j in (i + 1)..points.len() {
                        let point1 = points[i];
                        let point2 = points[j];
                        if point1 != point2 {
                            let antinodes = find_antinodes_for_pair(&grid, &point1, &point2);
                            for point in antinodes {
                                println!("Antinode {:?}", point);
                                results.push(point);
                            }
                        }
                    }
                }
                results
            })
            .collect();
        all.len() as i32
    }

    fn find_antinodes_for_pair(
        grid: &Vec<Vec<char>>,
        point1: &Point,
        point2: &Point,
    ) -> Vec<Point> {
        if point1.row > point2.row {
            return find_antinodes_for_pair(grid, point1, point2);
        }
        // point 1 is above point 2

        let row_distance = point2.row as i32 - point1.row as i32;
        let col_distance = point2.col as i32 - point1.col as i32;
        let slope = if row_distance < 0 && col_distance > 0 {
            Fraction::new((-row_distance) as u64, col_distance as u64).neg()
        } else if row_distance > 0 && col_distance < 0 {
            Fraction::new(row_distance as u64, (-col_distance) as u64).neg()
        } else {
            Fraction::new(row_distance as u64, col_distance as u64)
        };

        let numer = if slope.is_negative() {
            (*slope.numer().unwrap() as f64).neg()
        } else {
            *slope.numer().unwrap() as f64
        };
        let denom = *slope.denom().unwrap() as f64;

        println!("{point1:?}, {point2:?} slope {slope}");
        let mut result = vec![point1.clone(), point2.clone()];

        let mut multiplier = 1.0;
        loop {
            let row = point1.row as f64 - numer * multiplier;
            let col = point1.col as f64 - denom * multiplier;
            multiplier += 1.0;
            if row >= 0.0 && row < grid.len() as f64 && col >= 0.0 && col < grid[0].len() as f64 {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            } else {
                break;
            }
        }

        let mut multiplier = 1.0;
        loop {
            let row = point1.row as f64 + numer * multiplier;
            let col = point1.col as f64 + denom * multiplier;
            multiplier += 1.0;
            if row >= 0.0 && row < grid.len() as f64 && col >= 0.0 && col < grid[0].len() as f64 {
                let row = row as usize;
                let col = col as usize;
                result.push(Point { row, col });
            } else {
                break;
            }
        }

        result
    }
}
