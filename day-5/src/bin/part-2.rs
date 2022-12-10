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

type Stacks = Vec<Vec<String>>;
struct Move {
	count: usize,
	source: usize,
	target: usize,
}

fn parse_initial(s: &str) -> Result<Stacks, ParseError> {
	let mut lines: Vec<&str> = s.lines().collect();
	let mut stacks: Stacks = Vec::new();

	lines.reverse();
	for (i, c) in lines[0].chars().enumerate() {
		if c != ' ' {
			let mut stack: Vec<String> = Vec::new();
			for line in &lines[1..] {
				let chars: Vec<char> = line.chars().collect();
				if chars[i] == ' ' { break; }
				stack.push(chars[i].to_string());
			}
			stacks.push(stack);
		}
	}

	Ok(stacks)
}

fn parse_moves(s: &str) -> Result<Vec<Move>, ParseError> {
	let mut moves: Vec<Move> = Vec::new();

	for line in s.lines() {
		let words: Vec<&str> = line.split(" ").collect();
		let this_move = Move {
			count: words[1].parse::<usize>().map_err(|_| ParseError)?,
			source: words[3].parse::<usize>().map(|v| v - 1).map_err(|_| ParseError)?,
			target: words[5].parse::<usize>().map(|v| v - 1).map_err(|_| ParseError)?,
		};
		moves.push(this_move);
	}

	Ok(moves)
}

fn print_stacks(stacks: &Stacks) {
	for (i, stack) in stacks.iter().enumerate() {
		println!("{}: {}", i+1, stack.join(" "));
	}
}

fn apply_move(stacks: &mut Stacks, the_move: Move) {
	let source_stack = &mut stacks[the_move.source];
	let items = &mut source_stack.split_off(source_stack.len() - the_move.count);
	stacks[the_move.target].append(items);
}

fn main() {
	let file_contents = fs::read_to_string("input.txt")
		.expect("Failed to read file");

	let (initial_str, moves_str) = file_contents.split_once("\n\n")
		.expect("Failed to split input");

	let mut stacks = parse_initial(initial_str)
		.expect("Failed to parse initial configuration");
	let moves = parse_moves(moves_str)
		.expect("Failed to parse list of moves");

	for the_move in moves {
		apply_move(&mut stacks, the_move);
	}

	print_stacks(&stacks);

	println!("Answer: {}", stacks.iter()
		.map(|stack| &stack.last().expect("Empty stack in result")[..])
		.collect::<Vec<&str>>().join("")
	);
}
