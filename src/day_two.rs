pub fn part_one(input : &String) -> u32 {
	let bag_contents = CubeCount { red: 12, green: 13, blue: 14 };

	let mut id_sum : u32 = 0;

	let input_wo_whitespace = remove_whitespace(input.as_str());
	// split at "Game" and remove the first, because it is empty
	let games = input_wo_whitespace.split("Game").skip(1);

	for game in games {
		let mut id_and_game_info = game.split(':');

		let id = id_and_game_info.next()
			.expect("Text before ':' should exist")
			.parse::<u32>()
			.expect("Text before ':' should have been a number");

		let game_info = id_and_game_info.next().expect("Text after ':' should exist");

		let mut game_is_possible = true;
		for draw in game_info.split(';') {
			let cube_count = string_to_cube_count(draw);
			if !is_draw_possible(&cube_count, &bag_contents) {
				game_is_possible = false;
				break;
			}
		}

		if game_is_possible { id_sum += id }
	}

	id_sum
}

pub fn part_two(input : &String) -> u32 {
	let mut sum_of_powers : u32 = 0;

	let input_wo_whitespace = remove_whitespace(input.as_str());
	// split at "Game" and remove the first, because it is empty
	let games = input_wo_whitespace.split("Game").skip(1);

	for game in games {
		let game_info = game
			.split_once(':')
			.expect("Should have contained ':'")
			.1;

		let mut required_cubes = CubeCount { red: 0, green: 0, blue: 0 };
		for draw in game_info.split(';') {
			let cube_count = string_to_cube_count(draw);
			required_cubes = cube_count_max(&required_cubes, &cube_count);
		}

		let set_power = required_cubes.red * required_cubes.green * required_cubes.blue;
		sum_of_powers += set_power;
	}

	sum_of_powers
}

fn is_draw_possible(draw : &CubeCount, bag_contents : &CubeCount) -> bool {
	draw.red <= bag_contents.red
		&& draw.green <= bag_contents.green
		&& draw.blue <= bag_contents.blue
}

fn string_to_cube_count(s : &str) -> CubeCount {
	let mut cube_count = CubeCount { red: 0, green: 0, blue: 0 };

	let color_string_to_count = |s : &str, color : &str| -> u32 {
		match s.strip_suffix(color) {
			None => 0,
			Some(n) => n.parse::<u32>().expect("Should have been a number"),
		}
	};

	for color in s.split(',') {
		cube_count.red += color_string_to_count(color, "red");
		cube_count.green += color_string_to_count(color, "green");
		cube_count.blue += color_string_to_count(color, "blue");
	}

	cube_count
}

#[derive(Debug)]
struct CubeCount {
	red : u32,
	green : u32,
	blue : u32,
}

fn cube_count_max(left : &CubeCount, right : &CubeCount) -> CubeCount {
	CubeCount {
		red: std::cmp::max(left.red, right.red),
		green: std::cmp::max(left.green, right.green),
		blue: std::cmp::max(left.blue, right.blue),
	}
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part_one() {
		let sample_data =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
		assert_eq!(crate::day_two::part_one(&String::from(sample_data)), 8);
	}

	#[test]
	fn part_two() {
		let sample_data =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
		assert_eq!(crate::day_two::part_two(&String::from(sample_data)), 2286);
	}
}
