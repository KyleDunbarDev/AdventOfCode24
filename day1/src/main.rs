// curl --cookie "session=$COOKIE" https://adventofcode.com/2024/day/$DAY/input -O
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_distance() {
        assert_eq!(1, super::get_distance(5, 4));
    }
}
