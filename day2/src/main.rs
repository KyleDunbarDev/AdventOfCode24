use std::fs::read_to_string;

fn main() {
    let result = read_to_string("./src/input")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let ints: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            test_validity(&ints)
        })
        .filter(|&safe| safe)
        .count();
    println!("{}", result)
}

pub fn test_validity(ints: &Vec<i32>) -> bool {
    if ints.len() < 2 {
        return true;
    }

    let mut ascending = None;

    for i in 1..ints.len() {
        let diff = ints[i] - ints[i - 1];

        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        match ascending {
            None => {
                ascending = Some(diff > 0);
            }
            Some(ascend) => {
                if (diff > 0) != ascend {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_stuff() {
        //assert!()
    }
}
