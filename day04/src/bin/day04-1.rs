use array2d::Array2D;
use std::fs;

fn main() {
    let path = "day04-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn check(map: &Array2D<char>, pos: (i32, i32)) -> bool {
    let mut threshold = 0;
    let surround = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ];
    for p in surround {
        if p.0 < 0 || p.1 < 0 {
            continue;
        }
        let val = map.get(p.0 as usize, p.1 as usize);
        if val.unwrap_or(&'.') == &'@' {
            threshold += 1;
        }
    }
    return threshold < 4;
}

pub fn process(input: String) -> u64 {
    let mut result: u64 = 0;
    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let map = Array2D::from_rows(&rows).unwrap();
    for (pos, _) in map.enumerate_row_major().filter(|c| c.1 == &'@') {
        let accessable = check(&map, (pos.0 as i32, pos.1 as i32));
        if accessable {
            result += 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1_test() {
        let path = "day04-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 13);
    }
}
