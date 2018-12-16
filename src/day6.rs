use super::files;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_coordinate_list(lines: Vec<String>) -> (Vec<(i32, i32)>, i32, i32, i32, i32) {
    let coordinates_regex = Regex::new(r"^(\d+), (\d+)$").unwrap();

    // prepare coordinate_list and contour
    let mut coordinate_list: Vec<(i32, i32)> = Vec::new();
    let mut min_x = std::i32::MAX;
    let mut max_x = 0;
    let mut min_y = std::i32::MAX;
    let mut max_y = 0;
    for line in lines {
        for regs in coordinates_regex.captures_iter(&line) {
            let coordinates: (i32, i32) = (regs[1].parse().unwrap(), regs[2].parse().unwrap());
            if coordinates.0 < min_x {
                min_x = coordinates.0;
            }
            if coordinates.0 > max_x {
                max_x = coordinates.0;
            }
            if coordinates.1 < min_y {
                min_y = coordinates.0;
            }
            if coordinates.1 > max_y {
                max_y = coordinates.1;
            }
            coordinate_list.push(coordinates)
        }
    }
    return (coordinate_list, min_x, max_x, min_y, max_y);
}

pub fn task_1() {
    let input_file = "inputs/6.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let (coordinate_list, min_x, max_x, min_y, max_y) = get_coordinate_list(lines);

    let mut coordinates_map: HashMap<(i32, i32), ((i32, i32), bool)> = HashMap::new();
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut min_d = max_x - min_x + max_y - min_y;
            let is_on_edge = x == min_x || x == max_x || y == min_y || y == max_y;
            for coordinate in coordinate_list.iter() {
                let d = (x - coordinate.0).abs() + (y - coordinate.1).abs();
                if d == 0 {
                    coordinates_map.insert((x, y), (*coordinate, is_on_edge));
                    break;
                }
                if d < min_d {
                    min_d = d;
                    coordinates_map.insert((x, y), (*coordinate, is_on_edge));
                } else if d == min_d {
                    coordinates_map.remove(&(x, y));
                }
            }
        }
    }

    let mut coordinates_count: HashMap<(i32, i32), i32> = HashMap::new();
    let mut coordinate_ban: HashSet<(i32, i32)> = HashSet::new();
    let mut max = 0;
    for (coordinate, is_on_edge) in coordinates_map.values() {
        if coordinate_ban.contains(coordinate) {
            continue;
        }
        if *is_on_edge {
            coordinate_ban.insert(*coordinate);
            continue;
        }
        let new_count = *coordinates_count.entry(*coordinate).or_insert(0) + 1;
        coordinates_count.insert(*coordinate, new_count);
        if new_count > max {
            max = new_count;
        }
    }
    println!("coordinates_count {:?} ", coordinates_count);
    println!("coordinate_ban {:?} ", coordinate_ban);
    println!("max {:?} ", max);
}

pub fn task_2() {
    let input_file = "inputs/6.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let (coordinate_list, min_x, max_x, min_y, max_y) = get_coordinate_list(lines);

    let mut region_size = 0;
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut total_distance = 0;
            for coordinate in coordinate_list.iter() {
                let current_distance = (x - coordinate.0).abs() + (y - coordinate.1).abs();
                total_distance += current_distance;
            }
            if total_distance < 10000 {
                region_size += 1
            }
        }
    }

    println!("region size: {}", region_size);
}
