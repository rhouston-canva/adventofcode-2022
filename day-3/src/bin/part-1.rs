use std::collections::HashSet;
use std::fs;

use day_3::util;


fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let mut score = 0;
	for (line_number, line) in file_contents.lines().enumerate() {
		let chars: Vec<char> = line.chars().collect();
		let n = chars.len();
		if n % 2 != 0 {
			panic!("Line {} has an odd number of characters: {}", line_number + 1, line);
		}

		let a: HashSet<char> = HashSet::from_iter(chars[..n/2].iter().copied());
		let b: HashSet<char> = HashSet::from_iter(chars[n/2..].iter().copied());

		let intersection: Vec<char> = a.intersection(&b).copied().collect();
		
		if intersection.len() != 1 {
			panic!("Line {} has {} common characters", line_number + 1, intersection.len());
		}

		score += util::letter_score(intersection[0]);
	}

	println!("Score = {}", score);
}
