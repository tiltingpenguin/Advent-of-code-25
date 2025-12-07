use array2d::Array2D;
use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "day07-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn find_split(
    arr: &Array2D<char>,
    index: (usize, usize),
    mut splits: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let slice = arr.column_iter(index.1).unwrap().collect::<Vec<&char>>();
    let split = slice[index.0..].iter().position(|c| **c == '^');
    match split {
        Some(row) => {
            if splits.contains(&(index.0 + row, index.1)) {
                return splits;
            }
            splits.insert((index.0 + row, index.1));
            if index.1 > 0 {
                splits = find_split(arr, (index.0 + row + 1, index.1 - 1), splits);
            }
            if index.1 < arr.row_len() - 1 {
                splits = find_split(arr, (index.0 + row + 1, index.1 + 1), splits);
            }
        }
        None => (),
    }
    return splits;
}

pub fn process(input: String) -> usize {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .expect("Shoud be able to cast into 2D Array");
    let start = map.row_iter(0).unwrap().position(|c| *c == 'S').unwrap();
    let splits: HashSet<(usize, usize)> = HashSet::new();
    let count = find_split(&map, (0, start), splits);
    return count.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1_test() {
        let path = "day07-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 21);
    }
}
