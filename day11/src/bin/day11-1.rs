use pathfinding::prelude::count_paths;
use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "day11-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> usize {
    let graph = input
        .lines()
        .map(|line| {
            let (node, s) = line.split_once(": ").unwrap();
            let successors = s
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            (node.to_string(), successors)
        })
        .collect::<HashMap<String, Vec<String>>>();
    let path_len = count_paths(
        &"you".to_string(),
        |p| graph.get(p.clone()).unwrap().iter(),
        |p| *p == &"out".to_string(),
    );
    return path_len;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        let path = "day11-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 5);
    }
}
