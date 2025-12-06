use std::fs;
use std::iter;

fn main() {
    let path = "day06-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

pub fn process(input: String) -> u64 {
    let mut results: Vec<u64> = vec![];
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut cols = transpose(lines);
    let l = cols[0].len();
    cols.push(vec![' '; l]);
    let mut cur_problem: Vec<u64> = vec![];
    let mut last_operand = '*';

    for mut col in cols {
        // update the operand if there is a new one
        if col.last().unwrap() != &' ' {
            last_operand = *col.last().unwrap();
        }
        // if column is empty, save result of the current problem
        if col == vec![' '; l] {
            match last_operand {
                '*' => results.push(cur_problem.iter().fold(1, |acc, x| acc * x)),
                '+' => results.push(cur_problem.iter().sum()),
                _ => unreachable!(),
            }
            cur_problem = vec![];
        } else {
            col.pop();
            let num = col
                .iter()
                .filter(|c| **c != ' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            cur_problem.push(num);
        }
    }
    return results.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part2_test() {
        let path = "day06-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 3263827);
    }
}
