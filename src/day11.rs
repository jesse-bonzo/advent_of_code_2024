use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let day = 11;
    let part1_example_solution = 55312;

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
    println!("Solution Part2: {}", part2::solve(&input));
}

pub fn solve_for(input: &String, count: i32) -> i64 {
    let commands: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let mut cache: HashMap<(String, i32), i64> = HashMap::new();
    let mut result = 0;
    for command in commands.iter() {
        dbg!(&command);
        result += calculate(command, count, &mut cache);
    }
    result
}

fn calculate(command: &String, count: i32, cache: &mut HashMap<(String, i32), i64>) -> i64 {
    let cache_key = (command.clone(), count);
    let cached_result = cache.get(&cache_key);
    if cached_result.is_some() {
        return *cached_result.unwrap();
    }

    let result = if count <= 0 {
        1
    } else if command == "0" {
        calculate(&"1".to_string(), count - 1, cache)
    } else if command.len() % 2 == 0 {
        let (left, right) = command.split_at(command.len() / 2);
        calculate(&left.parse::<i64>().unwrap().to_string(), count - 1, cache)
            + calculate(&right.parse::<i64>().unwrap().to_string(), count - 1, cache)
    } else {
        calculate(
            &(command.parse::<i64>().unwrap() * 2024).to_string(),
            count - 1,
            cache,
        )
    };
    cache.insert(cache_key, result);
    result
}

mod part1 {
    use crate::day11::solve_for;

    pub fn solve(input: &String) -> i64 {
        solve_for(input, 25)
    }
}

mod part2 {
    use crate::day11::solve_for;

    pub fn solve(input: &String) -> i64 {
        solve_for(input, 75)
    }
}
