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
            //test_validity (&ints) //part 1
            test_validity_dampened(&ints) //part 2
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

pub fn test_validity_dampened(ints: &Vec<i32>) -> bool {
    //If not valid: clone list of ints. Remove an element. Test validity. Repeat
    if test_validity(ints) {
        return true;
    }

    for i in 0..ints.len() {
        let mut changed_ints = ints.clone();
        changed_ints.remove(i);

        if test_validity(&changed_ints) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_stuff() {
        //assert!()
    }
}
