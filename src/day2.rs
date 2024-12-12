#![allow(dead_code)]
use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day2_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 2 {
        println!("Example solution not valid for part1");
        return;
    }
    if part2::solve(&example_input) != 4 {
        println!("Example solution not valid for part2");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day2.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));
    println!("Solution Part2: {}", part2::solve(&input));
}

fn read_input(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line: &str| {
            return line
                .split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect();
        })
        .collect()
}

fn is_safe(group: &Vec<i32>) -> bool {
    println!("{:?}", group);
    if group.is_sorted() || group.iter().rev().is_sorted() {
        group.windows(2).all(|window| {
            let first = window[0];
            let second = window[1];
            let diff = (first - second).abs();
            return diff >= 1 && diff <= 3;
        })
    } else {
        false
    }
}

mod part1 {
    use crate::day2::{is_safe, read_input};

    pub fn solve(input: &String) -> i32 {
        let values = read_input(input);
        values.iter().filter(|&group| is_safe(group)).count() as i32
    }
}

mod part2 {
    use crate::day2::is_safe;

    pub fn solve(input: &String) -> i32 {
        let values = crate::day2::read_input(input);
        let mut safe_count = 0;
        for group in values {
            for i in 0..group.len() + 1 {
                let mut smaller_group = Vec::new();
                for j in 0..group.len() {
                    if i != j {
                        smaller_group.push(group[j]);
                    }
                }

                if is_safe(&smaller_group) {
                    safe_count += 1;
                    break;
                }
            }
        }
        safe_count
    }
}
