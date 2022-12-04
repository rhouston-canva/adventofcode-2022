use std::collections::HashMap;
use std::fs;

enum RPS {
	Rock,
	Paper,
	Scissors,
}

fn payoff(opp: &RPS, me: &RPS) -> i32 {
	match (opp, me) {
		(RPS::Rock, RPS::Rock) => 4,
		(RPS::Rock, RPS::Paper) => 8,
		(RPS::Rock, RPS::Scissors) => 3,

		(RPS::Paper, RPS::Rock) => 1,
		(RPS::Paper, RPS::Paper) => 5,
		(RPS::Paper, RPS::Scissors) => 9,

		(RPS::Scissors, RPS::Rock) => 7,
		(RPS::Scissors, RPS::Paper) => 2,
		(RPS::Scissors, RPS::Scissors) => 6,
	}
}


fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let opponent_moves = HashMap::from([
		("A", RPS::Rock), ("B", RPS::Paper), ("C", RPS::Scissors)
	]);
	let my_moves = HashMap::from([
		("X", RPS::Rock), ("Y", RPS::Paper), ("Z", RPS::Scissors)
	]);

	let mut sum = 0;
	for (line_number, line) in file_contents.lines().enumerate() {
		let (a, b) = line.split_once(" ")
			.expect(&format!("Failed to parse line {}: {}", line_number + 1, line));

		let opp = opponent_moves.get(a).unwrap();
		let me = my_moves.get(b).unwrap();

		sum += payoff(opp, me);
	}

	println!("Sum = {}", sum);
}