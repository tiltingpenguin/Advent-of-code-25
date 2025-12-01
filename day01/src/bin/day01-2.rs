use std::fs;

fn main() {
    let path = "day01-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> i32 {
    let mut result: i32 = 0;
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
        let tmp: i32 = cur + num;
        result += tmp.abs() / 100;
        if tmp <= 0 && cur > 0 {
            result += 1;
        }
        cur = tmp % 100;
        if cur < 0 {
            cur = 100 + cur;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part2_test() {
        let path = "day01-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 6);
    }
}
