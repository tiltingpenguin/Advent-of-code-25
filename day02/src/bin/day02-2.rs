use std::fs;

fn main() {
    let path = "day02-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u64 {
    let mut results: Vec<u64> = Vec::new();
    let ranges: Vec<(u64, u64)> = input
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|tup| {
            let s = tup.0.parse::<u64>().unwrap();
            let e = tup.1.parse::<u64>().unwrap();
            return (s, e);
        })
        .collect();
    for range in ranges {
        for num in range.0..range.1 + 1 {
            let tmp = num.to_string();
            let mid = tmp.len() / 2;
            for len in 1..mid + 1 {
                if tmp.len() % len == 0 {
                    let (pat, rest) = tmp.split_at(len);
                    if pat.repeat(rest.len() / len) == rest {
                        results.push(num);
                        break;
                    }
                }
            }
        }
    }

    return results.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part2_test() {
        let path = "day02-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 4174379265);
    }
}
