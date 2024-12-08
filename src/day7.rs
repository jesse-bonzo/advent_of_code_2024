use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day7_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 3749 {
        println!("Example solution not valid for part1");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day7.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));

    if part2::solve(&example_input) != 11387 {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

struct Equation {
    result: i64,
    inputs: Vec<i64>,
}

fn push_all(mut input: &mut Vec<i64>, to_push: Vec<i64>) {
    for i in to_push {
        input.push(i);
    }
}

mod part1 {
    use crate::day7::{push_all, Equation};

    pub(crate) fn solve(input: &String) -> i64 {
        input
            .lines()
            .map(|line| line.split(':').collect::<Vec<&str>>())
            .filter(|line| line.len() == 2)
            .map(|line| {
                let result: i64 = line.first().map(|it| it.parse::<i64>()).unwrap().unwrap();
                let inputs: Vec<_> = line
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|it| it.parse::<i64>().unwrap())
                    .collect();
                Equation { result, inputs }
            })
            .map(|equation: Equation| {
                let results = explore(&equation);
                if results.contains(&equation.result) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum()
    }

    fn explore(equation: &Equation) -> Vec<i64> {
        if equation.inputs.is_empty() {
            return vec![];
        } else if equation.inputs.len() == 1 {
            return vec![equation.inputs[0]];
        } else if equation.inputs.len() == 2 {
            let (a, b) = (equation.inputs[0], equation.inputs[1]);
            let mut result = vec![];
            match a.checked_add(b) {
                None => {}
                Some(sum) => {
                    result.push(sum);
                }
            }
            match a.checked_mul(b) {
                None => {}
                Some(product) => result.push(product),
            }
            return result;
        }

        let mut reduced_inputs: Vec<i64> = equation.inputs.clone();
        let input = reduced_inputs.pop().unwrap();
        let partial_results = explore(&Equation {
            result: equation.result,
            inputs: reduced_inputs,
        });
        let sums: Vec<i64> = partial_results
            .iter()
            .map(|it| it.checked_add(input))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        let products: Vec<i64> = partial_results
            .iter()
            .map(|it| it.checked_mul(input))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        let mut results: Vec<i64> = Vec::with_capacity(sums.len() + products.len());
        push_all(&mut results, sums);
        push_all(&mut results, products);
        results
    }
}

mod part2 {
    use crate::day7::{push_all, Equation};

    pub(crate) fn solve(input: &String) -> i64 {
        input
            .lines()
            .map(|line| line.split(':').collect::<Vec<&str>>())
            .filter(|line| line.len() == 2)
            .map(|line| {
                let result: i64 = line.first().map(|it| it.parse::<i64>()).unwrap().unwrap();
                let inputs: Vec<_> = line
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|it| it.parse::<i64>().unwrap())
                    .collect();
                Equation { result, inputs }
            })
            .map(|equation: Equation| {
                let results = explore(&equation);
                if results.contains(&equation.result) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum()
    }

    fn explore(equation: &Equation) -> Vec<i64> {
        if equation.inputs.is_empty() {
            return vec![];
        } else if equation.inputs.len() == 1 {
            return vec![equation.inputs[0]];
        } else if equation.inputs.len() == 2 {
            let (a, b) = (equation.inputs[0], equation.inputs[1]);
            let mut result = vec![];

            result.push(concat(a, b));

            match a.checked_add(b) {
                None => {}
                Some(sum) => {
                    result.push(sum);
                }
            }
            match a.checked_mul(b) {
                None => {}
                Some(product) => result.push(product),
            }
            return result;
        }

        let mut reduced_inputs: Vec<i64> = equation.inputs.clone();
        let input = reduced_inputs.pop().unwrap();
        let partial_results = explore(&Equation {
            result: equation.result,
            inputs: reduced_inputs,
        });
        let concats: Vec<i64> = partial_results
            .iter()
            .map(|it| concat(*it, input))
            .collect();
        let sums: Vec<i64> = partial_results
            .iter()
            .map(|it| it.checked_add(input))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        let products: Vec<i64> = partial_results
            .iter()
            .map(|it| it.checked_mul(input))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        let mut results: Vec<i64> = Vec::with_capacity(concats.len() + sums.len() + products.len());
        push_all(&mut results, concats);
        push_all(&mut results, sums);
        push_all(&mut results, products);
        results
    }

    fn concat(first: i64, second: i64) -> i64 {
        format!("{}{}", first, second).parse::<i64>().unwrap()
    }
}
