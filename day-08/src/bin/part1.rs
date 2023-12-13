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

#[derive(Debug)]
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
    let mut current_node = &nodes[0];

    let mut count = 0;
    while current_node.name != "ZZZ".to_string() {
        match &directions[count % directions.len()] {
            Direction::Right => {
                current_node = nodes
                    .iter()
                    .find(|node| node.name == current_node.right)
                    .unwrap()
            }
            Direction::Left => {
                current_node = nodes
                    .iter()
                    .find(|node| node.name == current_node.left)
                    .unwrap()
            }
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
