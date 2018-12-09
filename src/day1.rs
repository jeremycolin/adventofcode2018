use super::files;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;

pub fn task_1() {
    let input_file = "inputs/1.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let frequency_regex = Regex::new(r"^([+-])(\d+)$").unwrap();
    let mut current_frequency = 0;

    for line in lines {
        for freq in frequency_regex.captures_iter(&line) {
            let frequency_change: i32 = freq[2].parse().unwrap();
            if freq[1].eq("-") {
                current_frequency -= frequency_change
            } else {
                current_frequency += frequency_change
            }
        }
    }

    println!("Result frequency {}", current_frequency);
}

pub fn task_2() {
    let input_file = "inputs/1.1.txt".to_string();
    let frequency_regex = Regex::new(r"^([+-])(\d+)$").unwrap();

    let mut current_frequency = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut found = false;

    while !found {
        let lines = files::read_lines(&input_file);
        for line in lines {
            for freq in frequency_regex.captures_iter(&line) {
                let frequency_change: i32 = freq[2].parse().unwrap();
                if freq[1].eq("-") {
                    current_frequency -= frequency_change
                } else {
                    current_frequency += frequency_change
                }
                if frequencies.contains(&current_frequency) {
                    println!("Duplicate frequency  found {}", current_frequency);
                    found = true;
                }
                frequencies.insert(current_frequency);
            }
            if found {
                break;
            }
        }
    }
}
