use glam::I64Vec3;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Connection {
    distance: i64,
    boxes: (I64Vec3, I64Vec3),
}

fn main() {
    let path = "day08-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input, 1000);
    println!("{}", result);
}

pub fn process(input: String, len: usize) -> usize {
    let boxes: Vec<I64Vec3> = input
        .lines()
        .map(|line| {
            let tmp = line
                .split(",")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            I64Vec3::from_array(tmp.try_into().expect("Vec should be 3 elements long"))
        })
        .collect::<Vec<I64Vec3>>();
    // build list of graph edges
    let mut circuits: Vec<HashSet<I64Vec3>> = vec![];
    let mut connections: Vec<Connection> = vec![];
    for (index, b) in boxes.iter().enumerate() {
        if index == boxes.len() - 1 {
            break;
        }
        for sec in boxes[index + 1..].iter() {
            let dist = b.distance_squared(*sec);
            connections.push(Connection {
                distance: dist,
                boxes: (*b, *sec),
            });
        }
    }
    // find shortest connections and save as circuits
    connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    connections = connections[0..len].to_vec();
    for con in connections.iter() {
        let (box1, box2) = con.boxes;
        let mut in_circuit: bool = false;
        for c in &mut circuits {
            if c.contains(&box1) || c.contains(&box2) {
                c.insert(box1);
                c.insert(box2);
                in_circuit = true;
            }
        }
        if in_circuit == false {
            circuits.push(HashSet::from([box1, box2]));
        }
    }
    // merge circuits
    let mut flag = true;
    while flag {
        flag = false;
        let mut result = Vec::new();
        while let Some(mut circuit) = circuits.pop() {
            let mut i = 0;
            while i < circuits.len() {
                if !circuit.is_disjoint(&circuits[i]) {
                    let other = circuits.swap_remove(i);
                    circuit.extend(other);
                    flag = true;
                } else {
                    i += 1;
                }
            }
            result.push(circuit);
        }
        circuits = result;
    }
    circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    return circuits[0].len() * circuits[1].len() * circuits[2].len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1_test() {
        let path = "day08-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input, 10);
        assert_eq!(result, 40);
    }
}
