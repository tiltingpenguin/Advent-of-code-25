use std::fs;

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
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut problems = transpose(lines);
    return problems
        .iter_mut()
        .map(|p| {
            let operand = p.pop();
            match operand {
                Some("*") => return p.iter().fold(1, |acc, x| acc * x.parse::<u64>().unwrap()),
                Some("+") => return p.iter().map(|x| x.parse::<u64>().unwrap()).sum(),
                Some(_) => unreachable!("Only + and * are allowed operands"),
                None => unreachable!("Problem is empty"),
            }
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1_test() {
        let path = "day06-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 4277556);
    }
}
