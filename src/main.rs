use std::fs;

fn main() {
    let file_path = "input/day_one";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read input file");
    println!("Day One, Part One:\n{}\n", aoc_2023::day_one::part_one(&contents));
    println!("Day One, Part Two:\n{}\n", aoc_2023::day_one::part_two(&contents));

    let file_path = "input/day_two";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read input file");
    println!("Day One, Part One:\n{}\n", aoc_2023::day_two::part_one(&contents));
}
