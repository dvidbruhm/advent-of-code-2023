use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse_input(input: &str) -> (Vec<Direction>, BTreeMap<&str, (&str, &str)>) {
    let directions: Vec<Direction> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => Direction::Left,
            })
        })
        .collect();

    let mut map = BTreeMap::new();
    let _ = input
        .lines()
        .filter(|line| line.contains("="))
        .for_each(|line| {
            map.insert(&line[0..3], (&line[7..10], &line[12..15]));
        });
    (directions, map)
}

fn part1(input: &str) -> String {
    let (directions, map) = parse_input(input);
    let mut current_nodes: Vec<&&str> = map
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| node)
        .collect();

    let mut finished = false;
    let mut count = 0;

    let mut cycles: Vec<usize> = vec![];

    while finished == false {
        let current_dir = &directions[count % directions.len()];
        current_nodes
            .iter_mut()
            .for_each(|current_node| match current_dir {
                Direction::Right => *current_node = &map.get(*current_node).unwrap().1,
                Direction::Left => *current_node = &map.get(*current_node).unwrap().0,
            });

        count += 1;
        current_nodes = current_nodes
            .iter()
            .map(|node| {
                if node.ends_with('Z') {
                    cycles.push(count);
                }
                *node
            })
            .filter(|node| !node.ends_with('Z'))
            .collect();

        if current_nodes.is_empty() {
            finished = true;
        }
    }

    lcm(cycles.as_slice()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string());
    }
}
