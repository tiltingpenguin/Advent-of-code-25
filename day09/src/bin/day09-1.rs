use itertools::Itertools;
use std::fs;

fn main() {
    let path = "day09-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> i64 {
    let coords = input
        .lines()
        .map(|line| {
            let tup = line.split_once(",").unwrap();
            (tup.0.parse::<i64>().unwrap(), tup.1.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let areas = coords
        .into_iter()
        .combinations(2)
        .collect::<Vec<Vec<(i64, i64)>>>();
    let max = areas
        .iter()
        .map(|a| {
            return ((a[0].0 - a[1].0).abs() + 1) * ((a[0].1 - a[1].1).abs() + 1);
        })
        .max()
        .unwrap();
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1_test() {
        let path = "day09-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 50);
    }
}
