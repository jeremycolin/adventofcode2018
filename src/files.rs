use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_lines(input_path: &str) -> Vec<String> {
    // Create a path to the desired file
    let path = Path::new(input_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let buffer = BufReader::new(file);

    return buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}
