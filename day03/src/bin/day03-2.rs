use std::fs;

fn main() {
    let path = "day03-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn find_next_max(line: Vec<u64>, start: usize, end_offset: usize) -> (u64, usize) {
    let window = line[start..line.len() - end_offset].to_vec();
    // find max by value or smaller index
    let (max_index, max) = window
        .iter()
        .enumerate()
        .max_by(|x, y| {
            let ord = x.1.cmp(y.1);
            if ord.is_eq() {
                return x.0.cmp(&y.0).reverse();
            }
            return ord;
        })
        .unwrap();
    (*max, start + max_index)
}

pub fn process(input: String) -> u64 {
    let mut result: u64 = 0;
    let arr = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    for line in arr {
        let mut start = 0;
        let base: u64 = 10;
        for end_offset in (0..12).rev() {
            let (max, index) = find_next_max(line.clone(), start, end_offset);
            start = index + 1;
            result += max * base.pow(end_offset as u32);
        }
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
        assert_eq!(result, 3121910778619);
    }
}
