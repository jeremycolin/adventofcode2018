use super::files;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn prepare_require(lines: &Vec<String>) -> (HashMap<char, HashSet<char>>, HashSet<char>) {
    // Step C must be finished before step A can begin.
    let step_regex =
        Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
    let mut all_steps: HashSet<char> = HashSet::new();
    for line in lines {
        for steps in step_regex.captures_iter(&line) {
            let to = steps[1].parse().unwrap();
            let from = steps[2].parse().unwrap();
            let dependency = dependencies.entry(from).or_default();
            dependency.insert(to);
            all_steps.insert(from);
            all_steps.insert(to);
        }
    }

    println!("dependencies: {:?}", dependencies);
    return (dependencies, all_steps);
}

pub fn task_1() {
    let input_file = "inputs/7.1.txt".to_string();
    let lines = &files::read_lines(&input_file);
    let (mut dependencies, mut steps) = prepare_require(lines);

    let mut steps_to_process: Vec<char> = [].to_vec();
    while steps.len() > 0 {
        let mut next_steps: Vec<char> = steps
            .clone()
            .into_iter()
            .filter(|step| dependencies.clone().entry(*step).or_default().len() == 0)
            .collect();

        next_steps.sort();

        let chosen_step = next_steps[0];

        steps_to_process.push(chosen_step);
        // cleanup
        steps.remove(&chosen_step);
        for (_, steps) in dependencies.iter_mut() {
            steps.remove(&chosen_step);
        }
    }

    let steps_array: Vec<String> = steps_to_process.iter().map(|c| c.to_string()).collect();

    println!("steps: {:?}", steps_array.join(""));
}
