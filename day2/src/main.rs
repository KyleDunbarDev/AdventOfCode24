use std::fs::read_to_string;

fn main() {
    let result = read_to_string("./src/input")
        .unwrap()
        .lines()
        .split_whitespace()
        .collect()
        .map(test_validity)
        .sum();
}
pub fn test_validity(line: &str) => i32{
    for char in line{

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn find_stuff() {
        //assert!()
    }
}
