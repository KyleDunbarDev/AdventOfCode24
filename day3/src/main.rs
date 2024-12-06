use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let total_result: i32 = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty()) //every line thats not blank
        .map(|line| {
            // let line_results = find_valid_sequence(&line) //part 1
            let line_results = part_2(&line) //part 2
                .into_iter()
                .map(|int_tuple: (i32, i32)| int_tuple.0 * int_tuple.1)
                .collect::<Vec<i32>>()
                .iter()
                .sum::<i32>();
            line_results
        })
        .sum::<i32>();

    println!("{}", total_result);
}

pub fn find_valid_sequence(chars: &str) -> Vec<(i32, i32)> {
    println!("{}", chars);
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [first, second]) in re.captures_iter(chars).map(|c| c.extract()) {
        results.push((
            first.parse::<i32>().unwrap(),
            second.parse::<i32>().unwrap(),
        ));
    }
    // println!("{:?}", results);
    results
}

pub fn part_2(chars: &str) -> Vec<(i32, i32)> {
    let mut results: Vec<&str> = vec![];
    let mut valid_results: Vec<(i32, i32)> = vec![];

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let muls_re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    // let dos_re = Regex::new(r"(do\(\))").unwrap();
    // let donts__re = Regex::new(r"(don't\(\))").unwrap();

    for (_, [mul]) in muls_re.captures_iter(chars).map(|c| c.extract()) {
        results.push(mul);
    }

    let mut mul_indices: Vec<_> = vec![];
    for i in &results {
        mul_indices.push(chars.match_indices(i).last().unwrap());
    }
    let do_indices: Vec<_> = chars.match_indices("do()").collect();
    let dont_indices: Vec<_> = chars.match_indices("don't").collect();

    // println!("{:?}", results);
    // println!("{:?}", mul_indices);
    // println!("{:?}", do_indices);
    // println!("{:?}", dont_indices);
    let mut should_add: bool = false;

    for i in mul_indices {
        //nearest but not larger
        let mut nearest_do: i32 = 1;
        for j in &do_indices {
            if j.0 < i.0 && j.0 as i32 > nearest_do {
                nearest_do = j.0 as i32;
            }
        }
        let mut nearest_dont: i32 = 0;
        for h in &dont_indices {
            if h.0 < i.0 && h.0 as i32 > nearest_dont {
                nearest_dont = h.0 as i32;
            }
        }
        if nearest_do > nearest_dont {
            should_add = true;
            for (_, [first, second]) in re.captures_iter(i.1).map(|c| c.extract()) {
                valid_results.push((
                    first.parse::<i32>().unwrap(),
                    second.parse::<i32>().unwrap(),
                ));
            }
        } else {
            should_add = false
        }
        println!(
            "{:?}, {:?}, {:?}, {:?}",
            i, nearest_do, nearest_dont, should_add
        );
    }

    valid_results

    //Err / TODO: don'ts dont carry over from previous line
}

#[cfg(test)]
mod tests {
    use crate::find_valid_sequence;
    use crate::part_2;

    #[test]
    fn test_regex() {
        let chars: &str = "<;'mul(234,359)who()-wh";
        let expected: Vec<(i32, i32)> = vec![(234, 359)];
        println!("{:?}", find_valid_sequence(chars));
        assert!(find_valid_sequence(chars) == expected);
    }

    #[test]
    fn test_part2() {
        let chars: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected: Vec<(i32, i32)> = vec![(2, 4), (8, 5)];
        println!("{:?}", part_2(chars));
        assert!(part_2(chars) == expected);
    }
}
