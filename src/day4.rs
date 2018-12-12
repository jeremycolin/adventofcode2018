use super::files;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct ParsedLine {
    year: String,
    month: String,
    day: String,
    minutes: i32,
    hours: i32,
    action: Action,
}

#[derive(Debug)]
enum Action {
    WakeUp,
    FallAsleep,
    StartShift(i32),
}

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(parse_action<&str,Action>,
        alt_complete!(
            tag!("wakes") => { |_| Action::WakeUp } |
            tag!("falls") => { |_| Action::FallAsleep } |
            do_parse!(
                tag!("Guard #") >>
                guard: take_while!(is_digit) >>
                (
                    Action::StartShift(guard.parse().unwrap())
                )
            )
        )
    );

// [1518-11-01 00:00] Guard #10 begins shift
named!(parse_line<&str,ParsedLine>,
        ws!(
            do_parse!(
                         tag!("[") >>
                year:    take!(4) >>
                         tag!("-") >>
                month:   take!(2) >>
                         tag!("-") >>
                day:     take!(2) >>
                hours: take!(2) >>
                         tag!(":") >>
                minutes:   take!(2) >>
                         tag!("]") >>
                action: parse_action >>
                (
                    ParsedLine {
                        year: year.to_string(),
                        month: month.to_string(),
                        day: day.to_string(),
                        minutes: minutes.parse().unwrap(),
                        hours: hours.parse().unwrap(),
                        action: action
                    }
                )
            )
        )
    );

fn prepare_guard_map(
    mut lines: std::vec::Vec<std::string::String>,
) -> HashMap<i32, HashMap<String, HashSet<i32>>> {
    lines.sort();

    let logs = lines
        .iter()
        .filter_map(|line| parse_line(line).ok())
        .map(|x| x.1);

    let mut guard_map: HashMap<i32, HashMap<String, HashSet<i32>>> = HashMap::new();

    let mut current_guard = 0;
    let mut sleep_start = 0;
    for log in logs {
        // println!("{:?}", log);
        match log.action {
            Action::StartShift(guard) => {
                // println!("guard: {}", guard);
                current_guard = guard;
            }
            Action::FallAsleep => {
                sleep_start = log.minutes;
            }
            Action::WakeUp => {
                let guard_entry = guard_map.entry(current_guard).or_default();
                let day_schedule = guard_entry.entry(log.month + &log.day).or_default();
                for i in sleep_start..=log.minutes - 1 {
                    day_schedule.insert(i);
                }
            }
            _ => {}
        }
    }
    return guard_map;
}

pub fn task_1() {
    let input_file = "inputs/4.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let mut guard_map = prepare_guard_map(lines);
    let mut max_guard_asleep = 0;
    let mut sleeper = 0;

    for (guard, schedule) in guard_map.iter() {
        let mut current_guard_asleep = 0;
        for (_day, day_schedule) in schedule {
            current_guard_asleep += day_schedule.len();
        }
        if current_guard_asleep > max_guard_asleep {
            max_guard_asleep = current_guard_asleep;
            sleeper = *guard;
        }
    }

    println!("Sleeper is {}", sleeper);

    let mut minutes_map: HashMap<&i32, i32> = HashMap::new();
    for (_day, day_schedule) in guard_map.entry(sleeper).or_default() {
        for minute in day_schedule.iter() {
            *minutes_map.entry(minute).or_default() += 1
        }
    }

    let mut minutes_vec: Vec<_> = minutes_map.iter().collect();
    minutes_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!(
        "Most asleep at {} - Sleep count {}",
        minutes_vec[0].0, minutes_vec[0].1
    );
}

pub fn task_2() {
    let input_file = "inputs/4.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let guard_map = prepare_guard_map(lines);

    let mut sleeper = 0;
    let mut max_minute = 0;
    let mut max_minute_count = 0;
    for (guard, schedule) in guard_map.iter() {
        let mut minutes_map: HashMap<&i32, i32> = HashMap::new();
        for (_day, day_schedule) in schedule {
            for minute in day_schedule.iter() {
                *minutes_map.entry(minute).or_default() += 1
            }
        }

        let mut minutes_vec: Vec<_> = minutes_map.iter().collect();
        minutes_vec.sort_by(|a, b| b.1.cmp(a.1));

        if minutes_vec[0].1 > &max_minute_count {
            max_minute_count = *minutes_vec[0].1;
            max_minute = **minutes_vec[0].0;
            sleeper = *guard;
        }
    }

    println!(
        "sleeper is {} - Most asleep at {} - Sleep count {}",
        sleeper, max_minute, max_minute_count
    );
}
