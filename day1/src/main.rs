// curl --cookie "session=$COOKIE" https://adventofcode.com/2024/day/$DAY/input -O

use std::fs::read_to_string;

fn main() {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut distances: i32 = 0;

    //parse input into 2 lists
    let contents = read_to_string("./src/input.txt").unwrap();
    for line in contents.lines() {
        let ints: Vec<&str> = line.split_whitespace().collect();
        left.push(ints[0].parse::<i32>().unwrap());
        right.push(ints[1].parse::<i32>().unwrap());
    }

    //part 1
    left.sort();
    right.sort();
    let mut index = 0;
    for i in &left {
        distances += (i - right[index]).abs();
        index += 1;
    }
    println!("{}", distances);

    //part 2
    let similarity = left
        .iter()
        .map(|l| {
            let count = right.iter().filter(|r| l == *r).count();
            l * count as i32
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();
    println!("{}", similarity);
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_stuff() {
        //assert!()
    }
}
