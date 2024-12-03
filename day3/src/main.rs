use std::fs::read_to_string;
fn main() {
    let result = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty());
}
