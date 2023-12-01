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

use fancy_regex::Regex;

pub fn part_two(input : &String) -> u32 {
	let lines = input.split('\n');

	let mut sum = 0;

	for line in lines {
		// reached last line
		if line.len() == 0 { return sum }

		let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9))").unwrap();
		let mut matches = re.captures_iter(line).map(
			|result| result.expect("Error running regex").get(1).expect("No group").as_str()
		).peekable();

		let first = matches.next().expect("Line does not contain a digit");

		let two_digit_number;
		if matches.peek().is_none() {
			// only one number in line -> use as first and last digit
			two_digit_number = string_to_number(first) * 11;
		}
		else {
			let last = matches.last().unwrap();
			two_digit_number = string_to_number(first) * 10 + string_to_number(last);
		}

		sum += two_digit_number;
	}

	sum
}

fn string_to_number(input : &str) -> u32 {
	match input {
		"one" | "1" => 1,
		"two" | "2" => 2,
		"three" | "3" => 3,
		"four" | "4" => 4,
		"five" | "5" => 5,
		"six" | "6" => 6,
		"seven" | "7" => 7,
		"eight" | "8" => 8,
		"nine" | "9" => 9,
		_ => panic!("Input is neither a digit nor a spelled out digit: {}", input)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part_one() {
		let sample_data =
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
		assert_eq!(crate::day_one::part_one(&String::from(sample_data)), 142);
	}

	#[test]
	fn part_two() {
		let sample_data =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
		assert_eq!(crate::day_one::part_two(&String::from(sample_data)), 281);
	}

	#[test]
	fn part_two_overlapping_numbers() {
		// two overlapping numbers where the second one should succeed
		let sample_data = "fivezg8jmf6hrxnhgxxttwoneg";
		assert_eq!(crate::day_one::part_two(&String::from(sample_data)), 51);
	}
}
