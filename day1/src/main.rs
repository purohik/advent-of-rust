use std::fs;

const DEFAULT_PATH: &str = "input.txt";

fn part1(lines: &Vec<String>) -> i32 {
    for line in lines {
        let (first, last) = line.chars()
        .filter_map(|c| c.to_digit(10))
        .fold((None, None), |(f, _), x| (f.or(Some(x)), Some(x)));
    }
    0
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}

fn read_input(filepath: &str) -> Vec<String> {
    let contents: Vec<String> = fs::read_to_string(filepath)
        .expect("Should be reading the file ${filepath}")
        .lines()
        .map(|s| s.to_string())
        .collect();
    return contents;
}

fn main() {
    let input = read_input(DEFAULT_PATH);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}
