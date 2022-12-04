use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct ParseError;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}
impl Error for ParseError {}

fn parse(s: &str) -> Result<HashSet<i32>, ParseError> {
	let (x_str, y_str) = s.split_once("-").ok_or(ParseError)?;
	let x = x_str.parse::<i32>().map_err(|_| ParseError)?;
	let y = y_str.parse::<i32>().map_err(|_| ParseError)?;

	Ok(HashSet::from_iter(x..=y))
}

fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let mut score = 0;
	for (line_number, line) in file_contents.lines().enumerate() {
		let (a_str, b_str) = line.split_once(",")
			.expect(&format!("Failed to parse line {}: {}", line_number + 1, line));
		let a = parse(a_str).expect(&format!("Failed to parse '{}' on line {}", a_str, line_number + 1));
		let b = parse(b_str).expect(&format!("Failed to parse '{}' on line {}", b_str, line_number + 1));

		if a.is_superset(&b) || a.is_subset(&b) {
			score += 1;
		}
	}

	println!("Score = {}", score);
}
