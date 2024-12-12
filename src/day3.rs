#![allow(dead_code)]
use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day3_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 161 {
        println!("Example solution not valid for part1");
        return;
    }

    let example_input_2 = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day3_example_2.txt",
    )
    .expect("Unable to read input");
    if part2::solve(&example_input_2) != 48 {
        println!("Example solution not valid for part2");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day3.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));
    println!("Solution Part2: {}", part2::solve(&input));
}

mod part1 {
    use regex::Regex;

    pub fn solve(input: &String) -> i32 {
        let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();
        re.captures_iter(input)
            .map(|caps| {
                (
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                )
            })
            .map(|(x, y)| x * y)
            .sum()
    }
}

mod part2 {
    use regex::Regex;

    pub fn solve(input: &String) -> i32 {
        let re = Regex::new(r"((mul)\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
        let mut enabled = true;
        let mut sum = 0;
        for m in re.find_iter(input) {
            println!("{}", m.as_str());
            let x = m.as_str();
            if x.eq("do()") {
                enabled = true;
            } else if x.eq("don't()") {
                enabled = false;
            } else if enabled && x.contains("mul") {
                let first = x.split('(').collect::<Vec<&str>>()[1];
                let second = first.split(')').collect::<Vec<&str>>()[0];
                sum = sum
                    + second
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .reduce(|a, b| a * b)
                        .unwrap();
            }
        }
        sum
    }
}
