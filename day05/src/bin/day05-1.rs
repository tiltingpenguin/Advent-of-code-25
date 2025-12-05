use std::fs;

fn main() {
    let path = "day05-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u64 {
    let mut result = 0;
    let (fresh_ing, av_ing) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = fresh_ing
        .lines()
        .map(|line| {
            let (s, e) = line.split_once("-").unwrap();
            let start = s.parse::<u64>().unwrap();
            let end = e.parse::<u64>().unwrap();
            return (start, end);
        })
        .collect();
    let inv: Vec<u64> = av_ing
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    for item in inv {
        for range in ranges.iter() {
            if item >= range.0 && item <= range.1 {
                dbg!(item);
                result += 1;
                break;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1_test() {
        let path = "day05-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 3);
    }
}
