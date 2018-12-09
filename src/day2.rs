use super::files;
use std::collections::HashMap;

pub fn task_1() {
    let input_file = "inputs/2.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    let mut count_2: i32 = 0;
    let mut count_3: i32 = 0;

    for line in lines {
        let mut line_letters_map: HashMap<char, i32> = HashMap::new();

        let char_vec: Vec<char> = line.chars().collect();
        for c in char_vec {
            let c_count = *line_letters_map.entry(c).or_insert(0);
            line_letters_map.insert(c, c_count + 1);
        }

        let mut exact_2 = false;
        let mut exact_3 = false;

        for (&_letter, &count) in line_letters_map.iter() {
            if count == 2 {
                exact_2 = true;
            }
            if count == 3 {
                exact_3 = true;
            }
        }

        if exact_2 {
            count_2 += 1;
        }
        if exact_3 {
            count_3 += 1;
        }
    }

    println!("checksum {}", count_2 * count_3)
}

pub fn task_2() {
    let input_file = "inputs/2.1.txt".to_string();
    let lines = files::read_lines(&input_file);

    for i in 0..=lines.len() - 1 {
        let line_a_chars: Vec<char> = lines[i].chars().collect();
        for j in i + 1..=lines.len() - 1 {
            let line_b_chars: Vec<char> = lines[j].chars().collect();
            let mut differences = 0;
            for k in 0..=line_a_chars.len() - 1 {
                if line_a_chars[k] != line_b_chars[k] {
                    differences += 1;
                    if differences > 1 {
                        break;
                    }
                }
            }
            if differences < 2 {
                println!("woot!: {}, {}", lines[i], lines[j])
            }
        }
    }
}
