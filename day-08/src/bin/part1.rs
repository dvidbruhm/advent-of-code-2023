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
    let mut current_node = "AAA";

    let mut count = 0;
    while current_node != "ZZZ".to_string() {
        match &directions[count % directions.len()] {
            Direction::Right => current_node = map.get(current_node).unwrap().1,
            Direction::Left => current_node = map.get(current_node).unwrap().0,
        }
        count += 1;
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2".to_string());
    }
    #[test]
    fn test2() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6".to_string());
    }
}
