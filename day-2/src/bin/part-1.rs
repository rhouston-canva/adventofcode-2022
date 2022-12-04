use std::collections::HashMap;
use std::fs;


fn payoff(opp: i32, me: i32) -> i32 {
	me + (me - opp + 3) % 3 * 3
}

fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let opponent_moves = HashMap::from([ ("A", 0), ("B", 1), ("C", 2) ]);
	let my_moves = HashMap::from([ ("X", 1), ("Y", 2), ("Z", 3) ]);

	let mut sum = 0;
	for (line_number, line) in file_contents.lines().enumerate() {
		let (a, b) = line.split_once(" ")
			.expect(&format!("Failed to parse line {}: {}", line_number + 1, line));

		let opp = opponent_moves.get(a).unwrap();
		let me = my_moves.get(b).unwrap();
		sum += payoff(*opp, *me);
	}

	println!("Sum = {}", sum);
}