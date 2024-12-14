use regex::Regex;
use std::fs;

pub fn solve() {
    let day = 13;
    let part1_example_solution = 480;

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

    // let part2_example_solution = todo!();
    // if part2::solve(&example_input) != part2_example_solution {
    //     println!("Example solution not valid for part2");
    //     return;
    // }
    println!("Solution Part2: {}", part2::solve(&input));
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_location: Prize,
}

#[derive(Debug)]
struct Button {
    x_delta: usize,
    y_delta: usize,
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
}

fn read_input(input: &String) -> Vec<Machine> {
    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450
    let button_regex = Regex::new(r"^Button ([A|B]): X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
    let mut machines: Vec<Machine> = vec![];
    let mut button_a: Option<Button> = None;
    let mut button_b: Option<Button> = None;
    let mut prize_location: Option<Prize> = None;

    for line in input.lines() {
        if line.starts_with("Button") {
            match button_regex.captures(line) {
                Some(caps) => {
                    let button_name = caps.get(1).unwrap().as_str();
                    let x_delta = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    let y_delta = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
                    let button = Button { x_delta, y_delta };
                    match button_name {
                        "A" => button_a = Option::from(button),
                        "B" => button_b = Option::from(button),
                        _ => panic!("Unknown button name: {}", button_name),
                    }
                }
                None => panic!("Unable to parse line: {}", line),
            }
        }

        if line.starts_with("Prize") {
            prize_location = match prize_regex.captures(line) {
                Some(caps) => {
                    let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    Some(Prize { x, y })
                }
                None => panic!("Unable to parse line: {}", line),
            };
        }

        if prize_location.is_some() && button_a.is_some() && button_b.is_some() {
            machines.push(Machine {
                button_a: button_a.unwrap(),
                button_b: button_b.unwrap(),
                prize_location: prize_location.unwrap(),
            });
            prize_location = None;
            button_a = None;
            button_b = None;
        }
    }
    machines
}

mod part1 {
    use crate::day13::read_input;

    pub fn solve(input: &String) -> i64 {
        let button_a_cost = 3;
        let button_b_cost = 1;

        read_input(input)
            .iter()
            .map(|machine| {
                // ax*x + bx*x = px*x
                // ay*y + by*y = py*y
                let ax = machine.button_a.x_delta as i64;
                let bx = machine.button_b.x_delta as i64;
                let px = machine.prize_location.x as i64;
                let ay = machine.button_a.y_delta as i64;
                let by = machine.button_b.y_delta as i64;
                let py = machine.prize_location.y as i64;
                match check_linear_combination(ax, ay, bx, by, px, py) {
                    None => 0,
                    Some((a, b)) => a * button_a_cost + b * button_b_cost,
                }
            })
            .sum()
    }

    fn check_linear_combination(
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        cx: i64,
        cy: i64,
    ) -> Option<(i64, i64)> {
        if (ax * by) == (bx * ay) {
            panic!();
        }
        let n1 = (cx * by - cy * bx) / (ax * by - ay * bx);
        let n2 = (cx * ay - cy * ax) / (bx * ay - by * ax);
        if n1 * ax + n2 * bx != cx || n1 * ay + n2 * by != cy {
            None
        } else if n1 < 0 || n2 < 0 || n1 > 100 || n2 > 100 {
            None
        } else {
            Some((n1, n2))
        }
    }
}

mod part2 {
    use crate::day13::read_input;

    pub fn solve(input: &String) -> i64 {
        let button_a_cost = 3;
        let button_b_cost = 1;

        read_input(input)
            .iter()
            .map(|machine| {
                // ax*x + bx*x = px*x
                // ay*y + by*y = py*y
                let ax = machine.button_a.x_delta as i64;
                let bx = machine.button_b.x_delta as i64;
                let px = machine.prize_location.x as i64 + 10000000000000;
                let ay = machine.button_a.y_delta as i64;
                let by = machine.button_b.y_delta as i64;
                let py = machine.prize_location.y as i64 + 10000000000000;
                match check_linear_combination(ax, ay, bx, by, px, py) {
                    None => 0,
                    Some((a, b)) => a * button_a_cost + b * button_b_cost,
                }
            })
            .sum()
    }

    fn check_linear_combination(
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        px: i64,
        py: i64,
    ) -> Option<(i64, i64)> {
        dbg!(ax, ay, bx, by, px, py);
        if (ax * by) == (bx * ay) {
            panic!();
        }
        let n1 = (px * by - py * bx) / (ax * by - ay * bx);
        let n2 = (px * ay - py * ax) / (bx * ay - by * ax);
        dbg!(n1, n2);
        if n1 * ax + n2 * bx != px || n1 * ay + n2 * by != py {
            None
        } else if n1 < 0 || n2 < 0 {
            None
        } else {
            Some((n1, n2))
        }
    }
}
