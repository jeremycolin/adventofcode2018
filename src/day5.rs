use super::files;

fn get_alphas() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
}

fn collapse_chars(a: &char, b: &char) -> bool {
    return a.to_uppercase().eq(b.to_uppercase())
        && (a.is_uppercase() && b.is_lowercase() || a.is_lowercase() && b.is_uppercase());
}

fn compute_for_polymer(input_string: &String) -> usize {
    let input_vect: Vec<char> = input_string.chars().collect();
    let mut output_vect: Vec<char> = input_vect.clone();

    let mut index = 0;
    while index != output_vect.len() - 2 {
        if collapse_chars(&output_vect[index], &output_vect[index + 1]) {
            output_vect.remove(index);
            output_vect.remove(index);
            if index > 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }
    }

    let output_string: String = output_vect.iter().collect();
    println!("final string length: {}", output_string.len());
    return output_string.len();
}

pub fn task_1() {
    let input_file = "inputs/5.1.txt".to_string();
    let input_string = &files::read_lines(&input_file)[0];
    compute_for_polymer(input_string);
}

pub fn task_2() {
    let input_file = "inputs/5.1.txt".to_string();
    let input_string = &files::read_lines(&input_file)[0];

    let patterns_upper: Vec<String> = get_alphas()
        .into_iter()
        .map(|c| c.to_uppercase().to_string())
        .collect();

    let patterns_lower: Vec<String> = get_alphas().into_iter().map(|c| c.to_string()).collect();

    let mut min_length = input_string.len();
    let mut pattern_win = "";
    for it in patterns_upper.iter().zip(patterns_lower.iter()) {
        let mut string_to_process = input_string.clone();
        let (ai, bi) = it;
        string_to_process = string_to_process.replace(ai, "").replace(bi, "");
        let pattern_length = compute_for_polymer(&string_to_process);
        if pattern_length < min_length {
            min_length = pattern_length;
            pattern_win = ai;
        }
    }

    println!(
        "Best pattern was {} with length {}",
        pattern_win, min_length
    );
}
