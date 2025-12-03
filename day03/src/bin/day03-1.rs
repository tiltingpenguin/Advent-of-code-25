use std::fs;

fn main() {
    let path = "day03-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u32 {
    let mut result: u32 = 0;
    let arr = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    for line in arr {
        let max = line[..line.len() - 1].iter().max().unwrap();
        let max_index = line.clone().into_iter().position(|x| x == *max).unwrap();
        let sec = line[max_index + 1..].iter().max().unwrap();
        result += max * 10 + sec;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1_test() {
        let path = "day03-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 357);
    }
}
