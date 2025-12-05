use std::fs;

fn main() {
    let path = "day05-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u64 {
    let mut ranges: Vec<(u64, u64)> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let (s, e) = line.split_once("-").unwrap();
            let start = s.parse::<u64>().unwrap();
            let end = e.parse::<u64>().unwrap();
            return (start, end);
        })
        .collect();
    ranges.sort();
    let mut merged: Vec<(u64, u64)> = vec![];
    let mut tmp = merged.clone();
    let mut got_merged: bool;
    for r in ranges {
        got_merged = false;
        for (index, comp) in merged.iter().enumerate() {
            if r.0 <= comp.1 {
                if r.1 <= comp.1 {
                    got_merged = true;
                    break;
                } else {
                    tmp[index] = (comp.0, r.1);
                    got_merged = true;
                    break;
                }
            }
        }
        if got_merged == false {
            tmp.push(r);
        }
        merged = tmp.clone();
    }
    return merged
        .iter()
        .fold(0, |acc, (start, end)| acc + (end - start + 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part2_test() {
        let path = "day05-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 14);
    }
}
