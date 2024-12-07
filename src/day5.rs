use nohash_hasher::IntMap;
use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day5_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 143 {
        println!("Example solution not valid for part1");
        return;
    }

    if part2::solve(&example_input) != 123 {
        println!("Example solution not valid for part2");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day5.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));
    println!("Solution Part2: {}", part2::solve(&input));
}
fn read_input(input: &String, rules: &mut IntMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) {
    let mut in_rules_section = true;
    input.lines().map(|line| line.trim()).for_each(|line| {
        if line.is_empty() {
            in_rules_section = false;
        } else {
            if in_rules_section {
                let [first_page_number, second_page_number]: [i32; 2] = line
                    .split('|')
                    .map(|s| s.parse::<i32>().expect("Unable to parse input"))
                    .collect::<Vec<i32>>()
                    .try_into()
                    .expect("Unable to parse input");

                rules
                    .entry(first_page_number)
                    .or_default()
                    .push(second_page_number);
            } else {
                let page_numbers = line
                    .split(',')
                    .map(|word| word.parse::<i32>().expect("Unable to parse input"))
                    .collect::<Vec<i32>>();
                updates.push(page_numbers);
            }
        }
    });
}

fn is_valid(update: &Vec<i32>, rules: &IntMap<i32, Vec<i32>>) -> bool {
    // check if the update is in the right order based on the rules
    let any_rule_broken = (0..update.len()).any(|i| {
        let current_page_number = update[i];
        match rules.get(&current_page_number) {
            // check if any rules were broken
            Some(applicable_rules) => applicable_rules.iter().any(|&rule| {
                // the current_page_number must appear before the rule's number
                (0..i).any(|j| {
                    let previous_page_number = update[j];
                    // rule has been broken if this is true
                    previous_page_number == rule
                })
            }),
            None => false, // no rules to break
        }
    });
    !any_rule_broken
}

mod part1 {
    use crate::day5::{is_valid, read_input};
    use nohash_hasher::IntMap;

    pub fn solve(input: &String) -> i32 {
        let mut rules: IntMap<i32, Vec<i32>> = IntMap::default();
        let mut updates: Vec<Vec<i32>> = Vec::new();
        read_input(input, &mut rules, &mut updates);

        updates
            .iter()
            // get the valid updates
            .filter(|&update| is_valid(update, &rules))
            // now sum the middle numbers
            .map(|update| {
                let len = update.len();
                let mid = len / 2;
                update[mid]
            })
            .sum()
    }
}
mod part2 {
    use crate::day5::{is_valid, read_input};
    use nohash_hasher::IntMap;
    use std::cmp::Ordering;

    pub fn solve(input: &String) -> i32 {
        let mut rules: IntMap<i32, Vec<i32>> = IntMap::default();
        let mut updates: Vec<Vec<i32>> = Vec::new();
        read_input(input, &mut rules, &mut updates);

        updates
            .iter()
            // get the invalid updates
            .filter(|&update| !is_valid(update, &rules))
            // fix the updates to be in the right order
            .map(|update| {
                let mut page_numbers = update.to_vec();
                page_numbers.sort_by(|a, b| {
                    // find the rules for a
                    let a_before_b = match rules.get(a) {
                        // find a rule that says a must be before b
                        Some(list) => list.contains(b),
                        None => false,
                    };

                    return if a_before_b {
                        Ordering::Less
                    } else {
                        // no rule found so find the rules for b
                        let b_before_a = match rules.get(b) {
                            // find a rule that says b must be before a
                            Some(list) => list.contains(a),
                            None => false,
                        };
                        if b_before_a {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    };
                });
                page_numbers
            })
            // now sum the middle numbers
            .map(|update| {
                let len = update.len();
                let mid = len / 2;
                update[mid]
            })
            .sum()
    }
}
