use std::fs;

fn main() {
    let file_path = "input/day_one";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read input file");
    println!("Day One:\n{}\n", aoc_2023::day_one::day_one(contents));
}
