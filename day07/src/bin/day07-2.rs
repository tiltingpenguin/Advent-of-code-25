use array2d::Array2D;
use std::collections::HashMap;
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
    mut cache: HashMap<(usize, usize), u64>,
    mut count: u64,
) -> (HashMap<(usize, usize), u64>, u64) {
    let old_count = count;
    let slice = arr.column_iter(index.1).unwrap().collect::<Vec<&char>>();
    let split = slice[index.0..].iter().position(|c| **c == '^');
    match split {
        Some(row) => {
            if cache.contains_key(&(index.0 + row, index.1)) {
                return (
                    cache.clone(),
                    count + cache.get(&(index.0 + row, index.1)).unwrap(),
                );
            }
            count += 1;
            if index.1 > 0 {
                (cache, count) = find_split(arr, (index.0 + row + 1, index.1 - 1), cache, count);
            }
            if index.1 < arr.row_len() - 1 {
                (cache, count) = find_split(arr, (index.0 + row + 1, index.1 + 1), cache, count);
            }
            cache.insert((index.0 + row, index.1), count - old_count);
        }
        None => (),
    }
    return (cache, count);
}

pub fn process(input: String) -> u64 {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .expect("Shoud be able to cast into 2D Array");
    let start = map.row_iter(0).unwrap().position(|c| *c == 'S').unwrap();
    let splits: HashMap<(usize, usize), u64> = HashMap::new();
    let count = find_split(&map, (0, start), splits, 1);
    return count.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part2_test() {
        let path = "day07-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 40);
    }
}
