use super::files;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

pub fn task_1() {
    let input_file = "inputs/3.1.txt".to_string();
    let lines = files::read_lines(&input_file);
    let claim_regex = Regex::new(r"^#(\w+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claim_map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for regs in claim_regex.captures_iter(&line) {
            let x: i32 = regs[2].parse().unwrap();
            let y: i32 = regs[3].parse().unwrap();
            let width: i32 = regs[4].parse().unwrap();
            let height: i32 = regs[5].parse().unwrap();

            for i in x..=x + width - 1 {
                for j in y..=y + height - 1 {
                    let count = *claim_map.entry((i, j)).or_insert(0);
                    claim_map.insert((i, j), count + 1);
                }
            }
        }
    }

    let mut overlaps = 0;
    for (_claim, count) in claim_map {
        if count > 1 {
            overlaps += 1
        }
    }
    println!("Overlaps: {}", overlaps)
}

pub fn task_2() {
    let input_file = "inputs/3.1.txt".to_string();
    let lines = files::read_lines(&input_file);
    let claim_regex = Regex::new(r"^#(\w+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claim_map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for regs in claim_regex.captures_iter(&line) {
            let x: i32 = regs[2].parse().unwrap();
            let y: i32 = regs[3].parse().unwrap();
            let width: i32 = regs[4].parse().unwrap();
            let height: i32 = regs[5].parse().unwrap();

            for i in x..=x + width - 1 {
                for j in y..=y + height - 1 {
                    let count = *claim_map.entry((i, j)).or_insert(0);
                    claim_map.insert((i, j), count + 1);
                }
            }
        }
    }

    let lines = files::read_lines(&input_file);
    for line in lines {
        'id: for regs in claim_regex.captures_iter(&line) {
            let id: i32 = regs[1].parse().unwrap();
            let x: i32 = regs[2].parse().unwrap();
            let y: i32 = regs[3].parse().unwrap();
            let width: i32 = regs[4].parse().unwrap();
            let height: i32 = regs[5].parse().unwrap();

            for i in x..=x + width - 1 {
                for j in y..=y + height - 1 {
                    if *claim_map.entry((i, j)).or_default() > 1 {
                        break 'id;
                    }
                }
            }

            println!("No overlap for claim ID: {}", id);
        }
    }
}
