use std::collections::HashMap;
use std::fs;


fn payoff(opp: i32, me: i32) -> i32 {
	me + (me - opp + 3) % 3 * 3
}

fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let opponent_moves = HashMap::from([ ("A", 0), ("B", 1), ("C", 2) ]);
	let my_move_deltas = HashMap::from([ ("X", 2), ("Y", 0), ("Z", 1) ]);

	let mut sum = 0;
	for (line_number, line) in file_contents.lines().enumerate() {
		let (a, b) = line.split_once(" ")
			.expect(&format!("Failed to parse line {}: {}", line_number + 1, line));

		let opp = *opponent_moves.get(a).unwrap();
		let me = 1 + (opp + my_move_deltas.get(b).unwrap()) % 3;
		sum += payoff(opp, me);

		// println!("{}/{} = {}/{} = {}", a, b, opp, me, payoff(opp, me));
	}

	println!("Sum = {}", sum);
}