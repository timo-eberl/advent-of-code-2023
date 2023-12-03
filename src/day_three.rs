use std::collections::HashMap;

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

pub fn part_two(input : &String) -> u32 {
	// key: gear idx, value: list of adjacent part numbers
	let mut gears : HashMap<usize, Vec<u32>> = HashMap::new();

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

			let part_number : u32 = input[part_number_start_idx..part_number_end_idx]
				.parse::<u32>()
				.expect("Should have been a number");

			let part_number_adjacent_gears = part_number_find_adjacent_gears(
				&part_number_start_idx,
				&part_number_end_idx,
				input,
				&line_length,
			);

			for gear_idx in part_number_adjacent_gears {
				if let Some(part_numbers) = gears.get_mut(&gear_idx) {
					part_numbers.push(part_number);
				}
				else {
					gears.insert(gear_idx, vec![part_number]);
				}
			}

			part_number_started = false;
		}
	}

	let mut gear_ratios_sum = 0;

	for adjacent_part_numbers in gears.values() {
		if adjacent_part_numbers.len() == 2 {
			let gear_ratio = adjacent_part_numbers[0] * adjacent_part_numbers[1];
			gear_ratios_sum += gear_ratio;
		}
	}

	gear_ratios_sum
}

fn part_number_find_adjacent_gears(
	part_number_start_idx : &usize,
	part_number_end_idx : &usize,
	engine_schematic : &str,
	schematic_line_length : &usize,
) -> Vec<usize> {
	let mut gear_indices : Vec<usize> = Vec::new();

	// coordinates
	let pn_y = part_number_start_idx / schematic_line_length;
	let pn_start_x = part_number_start_idx % schematic_line_length;
	let pn_end_x = part_number_end_idx % schematic_line_length; // exclusive

	let area_start_x = match pn_start_x { 0 => 0, _ => pn_start_x - 1, };
	let area_end_x = std::cmp::min(schematic_line_length - 1, pn_end_x + 1);  // exclusive
	let area_start_y = match pn_y { 0 => 0, _ => pn_y - 1, };
	let has_next_line = (pn_y + 1) * schematic_line_length < engine_schematic.len();
	let area_end_y = match has_next_line { true => pn_y + 2, false => pn_y + 1, };  // exclusive

	let find_gear_indices = |s : &str| -> Vec<usize> {
		s
			.char_indices()
			.filter(|c| c.1 == '*')
			.map(|c| c.0)
			.collect()
	};

	for y in area_start_y..area_end_y {
		let idx_start = y * schematic_line_length + area_start_x;
		let idx_end = y * schematic_line_length + area_end_x;
		let line = &engine_schematic[idx_start..idx_end];
		let mut line_gear_indices = find_gear_indices(line)
			.iter()
			.map(|local_idx| idx_start + local_idx)
			.collect();
		
		gear_indices.append(&mut line_gear_indices);
	}
	
	gear_indices
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

	#[test]
	fn part_two() {
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
		assert_eq!(crate::day_three::part_two(&String::from(sample_data)), 467835);
	}
}
