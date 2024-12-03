use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let result = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty()) //every line thats not blank
        .map(|line| {
            let chars: Vec<&str> = line.split("").collect();
            find_valid_sequence(&chars);
        })
        .sum();

    println!("{}", result);
}

pub fn find_valid_sequence(chars: &Vec<&str>) -> Vec<i32> {
    let mut ints_left: Vec<i32> = vec![];
    let mut ints_right: Vec<i32> = vec![];

    return 0;
}
