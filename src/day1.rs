#![allow(dead_code)]
use std::fs;

pub fn solve() {
    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day1.txt",
    )
    .expect("Unable to read input");

    // part1::solve_for(&input);
    part2::solve_for(&input);
}

fn read_input(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();
    input.lines().for_each(|line| {
        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace() //
            .map(|s| s.parse::<i32>()) //
            .collect();

        match numbers {
            Ok(nums) => {
                if nums.len() == 2 {
                    first_col.push(nums[0]);
                    second_col.push(nums[1]);
                }
            }
            Err(e) => println!("Error parsing numbers: {}", e),
        }
    });
    (first_col, second_col)
}

mod part1 {
    use crate::day1::read_input;

    pub fn solve_for(input: &String) {
        let (mut first_col, mut second_col) = read_input(input);
        first_col.sort();
        second_col.sort();
        let mut total = 0;
        for i in 0..first_col.len() {
            let diff = first_col[i] - second_col[i];
            total += diff.abs();
        }
        println!("Total: {}", total);
    }
}

mod part2 {
    use crate::day1::read_input;

    pub fn solve_for(input: &String) {
        let (first_col, second_col) = read_input(input);
        let mut total = 0;
        first_col.iter().for_each(|&first_num| {
            let count = second_col
                .iter()
                .filter(|&second_num| *second_num == first_num)
                .count();
            let score = first_num * count as i32;
            total += score;
        });
        println!("Total: {}", total);
    }
}
