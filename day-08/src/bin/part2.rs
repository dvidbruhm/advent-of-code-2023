use std::io::Write;

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

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (Vec<Direction>, Vec<Node>) {
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

    let mut nodes: Vec<Node> = input
        .lines()
        .filter(|line| line.contains("="))
        .map(|line| Node {
            name: line[0..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        })
        .collect();
    nodes.sort_by(|a, b| a.name.cmp(&b.name));
    (directions, nodes)
}

fn part1(input: &str) -> String {
    let (directions, nodes) = parse_input(input);

    let mut current_nodes = nodes.clone();
    current_nodes.retain(|node| node.name.chars().last().unwrap() == 'A');
    let mut finished = false;
    let mut count = 0;

    while finished == false {
        print!("\r{}", count);
        current_nodes.iter_mut().for_each(|current_node| {
            match &directions[count % directions.len()] {
                Direction::Right => {
                    *current_node = nodes
                        .iter()
                        .find(|node| node.name == current_node.right)
                        .unwrap()
                        .clone()
                }
                Direction::Left => {
                    *current_node = nodes
                        .iter()
                        .find(|node| node.name == current_node.left)
                        .unwrap()
                        .clone()
                }
            }
        });
        finished = current_nodes
            .iter()
            .all(|node| node.name.chars().last().unwrap() == 'Z');
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
