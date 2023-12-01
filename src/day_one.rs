pub fn part_one(input : &String) -> u32 {
	let lines = input.split('\n');

	let mut sum = 0;

	for line in lines {
		// reached last line
		if line.len() == 0 { return sum }

		let mut two_digit_number_s = String::with_capacity(2);

		for character in line.chars() {
			if character.is_ascii_digit() {
				two_digit_number_s.insert(0, character);
				break
			}
		}

		for character in line.chars().rev() {
			if character.is_ascii_digit() {
				two_digit_number_s.insert(1, character);
				break
			}
		}

		let two_digit_number = two_digit_number_s.parse::<u32>()
			.expect("Could not parse line");
		sum += two_digit_number;
	}

	sum
}

pub fn part_two(_input : &String) -> u32 {
	0
}
