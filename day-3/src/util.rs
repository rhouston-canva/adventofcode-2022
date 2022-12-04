pub fn letter_score(letter: char) -> u32 {
	if letter >= 'a' && letter <= 'z' {
		return ((letter as u32) - ('a' as u32)) + 1;
	}

	if letter >= 'A' && letter <= 'Z' {
		return ((letter as u32) - ('A' as u32)) + 27;
	}
	panic!("Unexpected letter {}", letter);
}
