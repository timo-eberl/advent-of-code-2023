pub fn part_one(input : &String) -> u32 {
	let mut part_number_sum = 0;

	let line_length = input
		.find('\n')
		.expect("Should have contained a '\\n' character")
		+ 1; // '\n' counts to the same line

	let mut part_number_started = false;
	let mut part_number_start_idx = 0;
	for (i, c) in input.char_indices() {
		if c.is_ascii_digit() && !part_number_started {
			// start of part number
			part_number_start_idx = i;
			part_number_started = true;
		}
		
		if !c.is_ascii_digit() && part_number_started {
			// end of part number
			let part_number_end_idx = i;

			if part_number_has_adjacent_symbol(
				&part_number_start_idx,
				&part_number_end_idx,
				input,
				&line_length,
			) {
				let part_number : u32 = input[part_number_start_idx..part_number_end_idx]
					.parse::<u32>()
					.expect("Should have been a number");
				
				part_number_sum += part_number;
			}

			part_number_started = false;
		}
	}

	part_number_sum
}

fn part_number_has_adjacent_symbol(
	part_number_start_idx : &usize,
	part_number_end_idx : &usize,
	engine_schematic : &str,
	schematic_line_length : &usize,
) -> bool {
	// coordinates
	let pn_y = part_number_start_idx / schematic_line_length;
	let pn_start_x = part_number_start_idx % schematic_line_length;
	let pn_end_x = part_number_end_idx % schematic_line_length; // exclusive

	let area_start_x = match pn_start_x { 0 => 0, _ => pn_start_x - 1, };
	let area_end_x = std::cmp::min(schematic_line_length - 1, pn_end_x + 1);  // exclusive
	let area_start_y = match pn_y { 0 => 0, _ => pn_y - 1, };
	let has_next_line = (pn_y + 1) * schematic_line_length < engine_schematic.len();
	let area_end_y = match has_next_line { true => pn_y + 2, false => pn_y + 1, };  // exclusive

	let contains_symbol = |s : &str| -> bool {
		s
			.chars()
			.find(|c| !c.is_ascii_digit() && c != &'.' && c != &'\n')
			.is_some()
	};

	for y in area_start_y..area_end_y {
		let idx_start = y * schematic_line_length + area_start_x;
		let idx_end = y * schematic_line_length + area_end_x;
		let line = &engine_schematic[idx_start..idx_end];
		let line_contains_symbol = contains_symbol(line);

		if line_contains_symbol { return true; }
	}

	false
}

#[cfg(test)]
mod tests {
	#[test]
	fn part_one() {
		let sample_data =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
		assert_eq!(crate::day_three::part_one(&String::from(sample_data)), 4361);
	}
}
