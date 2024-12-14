use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let day = 12;
    let part1_example_solution = 1930;

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

    let part2_example_solution = 1206;
    if part2::solve(&example_input) != part2_example_solution {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

fn find_groups(plant_type: char, grid: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut matches: Vec<(usize, usize)> = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == plant_type {
                matches.push((row, col));
            }
        }
    }

    let mut groups: HashSet<Vec<(usize, usize)>> = HashSet::new();
    for match_ in matches.iter() {
        let mut match_neighbors: HashSet<(usize, usize)> = HashSet::new();
        match_neighbors.insert(*match_);

        let mut checked: HashSet<(usize, usize)> = HashSet::new();
        let mut to_explore = vec![match_];
        while !to_explore.is_empty() {
            let current = to_explore.pop().unwrap();
            if checked.insert(*current) {
                let neighbors: Vec<_> = matches
                    .iter()
                    .filter(|&pos| is_next_to(current, pos))
                    .collect();
                for n in neighbors {
                    match_neighbors.insert(*n);
                    to_explore.push(n);
                }
            }
        }

        let mut m: Vec<_> = match_neighbors
            .iter()
            .map(|(row, col)| (*row, *col))
            .collect();
        m.sort();
        groups.insert(m);
    }
    groups.iter().map(|group| group.to_vec()).collect()
}

fn is_next_to(first: &(usize, usize), second: &(usize, usize)) -> bool {
    let (first_row, first_col) = *first;
    let (second_row, second_col) = *second;

    if first_row == second_row {
        first_col + 1 == second_col || first_col == second_col + 1
    } else if first_col == second_col {
        first_row + 1 == second_row || first_row == second_row + 1
    } else {
        false
    }
}

mod part1 {
    use crate::day12::{find_groups, is_next_to};
    use std::collections::HashSet;

    pub fn solve(input: &String) -> i32 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut plant_types: HashSet<char> = HashSet::new();
        grid.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                plant_types.insert(*cell);
            })
        });

        plant_types
            .iter()
            .flat_map(|plant_type| find_groups(*plant_type, &grid))
            .map(|group| {
                let area = group.len();
                let perimeter = calculate_perimeter(group);
                area as i32 * perimeter
            })
            .sum()
    }

    fn calculate_perimeter(group: Vec<(usize, usize)>) -> i32 {
        let mut perimeter = 0;
        for plot in group.iter() {
            // this plot contributes 4 minus the number of plots touching to perimeter
            let neighbor_count = group.iter().filter(|other| is_next_to(plot, other)).count();
            perimeter += 4 - neighbor_count as i32;
        }
        perimeter
    }
}

mod part2 {
    use crate::day12::find_groups;
    use std::collections::HashSet;

    pub fn solve(input: &String) -> i32 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut plant_types: HashSet<char> = HashSet::new();
        grid.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                plant_types.insert(*cell);
            })
        });

        plant_types
            .iter()
            .map(|plant_type| {
                let groups = find_groups(*plant_type, &grid);
                let mut cost = 0;
                for group in groups {
                    if group.len() > 0 {
                        let area = group.len() as i32;
                        let sides = calculate_sides(&grid, &group);
                        cost += area * sides;
                    }
                }
                cost
            })
            .sum()
    }

    fn calculate_sides(grid: &Vec<Vec<char>>, group: &Vec<(usize, usize)>) -> i32 {
        let plant_type = grid[group[0].0][group[0].1];
        println!("calculate_sides: {:?}", plant_type);
        let mut sides = 0;

        let safe_get = |row: i32, col: i32| {
            if row >= 0
                && row < grid.len() as i32
                && col >= 0
                && col < grid[row as usize].len() as i32
            {
                Some(grid[row as usize][col as usize])
            } else {
                None
            }
        };

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if group.contains(&(row, col)) && grid[row][col] == plant_type {
                    let up_to_the_left = safe_get(row as i32 - 1, col as i32 - 1).unwrap_or(' ');
                    let up = safe_get(row as i32 - 1, col as i32).unwrap_or(' ');
                    let up_to_the_right = safe_get(row as i32 - 1, col as i32 + 1).unwrap_or(' ');
                    let left = safe_get(row as i32, col as i32 - 1).unwrap_or(' ');
                    let right = safe_get(row as i32, col as i32 + 1).unwrap_or(' ');
                    let down_to_the_left = safe_get(row as i32 + 1, col as i32 - 1).unwrap_or(' ');
                    let down = safe_get(row as i32 + 1, col as i32).unwrap_or(' ');
                    // let h = safe_get(row as i32 + 1, col as i32 + 1).unwrap_or(' ');

                    if left != plant_type && (up != plant_type || up_to_the_left == plant_type) {
                        sides += 1;
                    }
                    if right != plant_type && (up != plant_type || up_to_the_right == plant_type) {
                        sides += 1;
                    }
                    if up != plant_type && (left != plant_type || up_to_the_left == plant_type) {
                        sides += 1;
                    }
                    if down != plant_type && (left != plant_type || down_to_the_left == plant_type)
                    {
                        sides += 1;
                    }
                }
            }
        }
        dbg!(sides);
        sides
    }
}
