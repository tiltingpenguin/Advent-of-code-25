use std::fs;

fn main() {
    let path = "day01-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u64 {
    let mut result: u64 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let values: Vec<i32> = lines
        .iter()
        .map(|x| {
            let s: String = x.to_string();
            let (dir, amount) = s.split_at(1);
            let mut val: i32 = amount.parse().unwrap();
            match dir {
                "L" => val = val * -1,
                "R" => (),
                _ => unreachable!(),
            }
            return val;
        })
        .collect();
    let mut cur: i32 = 50;
    for num in values {
        cur = (cur + num) % 100;
        if cur < 0 {
            cur = 100 + cur;
        }
        if cur == 0 {
            result += 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_test() {
        let path = "day01-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 3);
    }
}
