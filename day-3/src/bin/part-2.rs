use itertools::Itertools;
use std::collections::HashSet;
use std::iter::Iterator;
use std::fs;

use day_3::util;

fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let mut score = 0;
	for chunk_it in &file_contents.lines().chunks(3) {
		let chunk: Vec<&str> = chunk_it.collect();

		let a: HashSet<char> = HashSet::from_iter(chunk[0].chars());
		let b: HashSet<char> = HashSet::from_iter(chunk[1].chars());
		let c: HashSet<char> = HashSet::from_iter(chunk[2].chars());

		let intersection: Vec<char> = a
			.intersection(&b).copied().collect::<HashSet<char>>()
			.intersection(&c).copied().collect();

		score += util::letter_score(intersection[0]);
	}

	println!("Score = {}", score);
}
