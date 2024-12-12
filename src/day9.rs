#![allow(dead_code)]
use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day9_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 1928 {
        println!("Example solution not valid for part1");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day9.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));

    if part2::solve(&example_input) != 2858 {
        println!("Example solution not valid for part2");
        return;
    }
    println!("Solution Part2: {}", part2::solve(&input));
}

mod part1 {
    pub fn solve(input: &String) -> i64 {
        let digits: Vec<i32> = input
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<i32>())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        let mut disk: Vec<i32> = Vec::new();
        let mut file_id = 0;
        for i in 0..digits.len() {
            if i % 2 == 0 {
                // file len
                let file_len = digits[i];
                for _j in 0..file_len {
                    disk.push(file_id)
                }
                file_id += 1;
            } else {
                // free space len
                let free_space_len = digits[i];
                for _j in 0..free_space_len {
                    disk.push(-1);
                }
            }
        }

        let mut i = 0;
        let mut j = disk.len() - 1;
        loop {
            while i < disk.len() && disk[i] != -1 {
                i += 1;
            }

            while disk[j] == -1 && j > 0 {
                j -= 1;
            }

            if i >= j {
                break;
            }

            let tmp = disk[i];
            disk[i] = disk[j];
            disk[j] = tmp;
        }

        let mut check_sum: i64 = 0;
        for i in 0..disk.len() {
            if disk[i] == -1 {
                break;
            }

            check_sum = check_sum + disk[i] as i64 * i as i64;
        }
        check_sum
    }
}

mod part2 {
    pub fn solve(input: &String) -> i64 {
        let digits: Vec<i32> = input
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<i32>())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        let mut disk: Vec<i32> = Vec::new();
        let mut file_ids: Vec<i32> = Vec::new();
        let mut file_id = 0;
        for i in 0..digits.len() {
            if i % 2 == 0 {
                // file len
                let file_len = digits[i];
                for _j in 0..file_len {
                    disk.push(file_id)
                }
                file_ids.push(file_id);
                file_id += 1;
            } else {
                // free space len
                let free_space_len = digits[i];
                for _j in 0..free_space_len {
                    disk.push(-1);
                }
            }
        }

        while !file_ids.is_empty() {
            let file_id = file_ids.pop().unwrap();

            // determine length needed
            // and find the first index of file_id
            let mut position: i32 = -1;
            let mut len = 0;
            for i in 0..disk.len() {
                if position == -1 && disk[i] == file_id {
                    position = i as i32;
                }

                if position != -1 {
                    if disk[i] == file_id {
                        len += 1;
                    } else {
                        break;
                    }
                }
            }

            if position < 0 || len < 0 {
                continue;
            }

            // should be okay to be a usize now
            let position = position as usize;
            let len = len as usize;

            // find a space that is len long
            let mut i = 0;
            while i < disk.len() && i < position {
                // skip over occupied space
                while i < disk.len() && disk[i] != -1 {
                    i += 1;
                }

                // check we didn't go to far
                if i >= position || i >= disk.len() {
                    break;
                }

                // figure out how long this empty space is and where it starts and ends
                let mut empty_len = 0;
                let empty_start = i;
                while disk[i] == -1 {
                    empty_len += 1;
                    i += 1;
                }
                let empty_end = empty_start + empty_len;

                if empty_len >= len {
                    // found a space big enough
                    let mut empty_index = empty_start;
                    let mut filled_index = position;
                    let filled_end = position + len;
                    while empty_index < empty_end && filled_index < filled_end {
                        let tmp = disk[empty_index];
                        disk[empty_index] = disk[filled_index];
                        disk[filled_index] = tmp;
                        empty_index += 1;
                        filled_index += 1;
                    }
                    break;
                }
            }
        }

        let mut check_sum: i64 = 0;
        for i in 0..disk.len() {
            if disk[i] != -1 {
                check_sum = check_sum + disk[i] as i64 * i as i64;
            }
        }
        check_sum
    }
}
