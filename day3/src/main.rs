use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let result: i32 = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty()) //every line thats not blank
        .map(|line| {
            let result = find_valid_sequence(&line)
                .into_iter()
                .map(|int_tuple: (i32, i32)| int_tuple.0 * int_tuple.1)
                .collect::<Vec<i32>>()
                .iter()
                .sum::<i32>();
            result
        })
        .sum::<i32>();

    println!("{}", result);
}

pub fn find_valid_sequence(chars: &str) -> Vec<(i32, i32)> {
    println!("{}", chars);
    let re = Regex::new(r"mul\\((\\d+),(\\d+)\\)").unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [first, second]) in re.captures_iter(chars).map(|c| c.extract()) {
        results.push((
            first.parse::<i32>().unwrap(),
            second.parse::<i32>().unwrap(),
        ));
    }
    println!("{:?}", results);
    results
}

#[cfg(test)]
mod tests {
    use crate::find_valid_sequence;

    #[test]
    fn test_regex() {
        let chars: &str = "<;'mul(234,359)who()-wh";
        let expected: Vec<(i32, i32)> = vec![(234, 359)];
        println!("{:?}", find_valid_sequence(chars));
        assert!(find_valid_sequence(chars) == expected);
    }
}
