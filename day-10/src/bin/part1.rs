use std::{collections::BTreeMap, vec};

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (BTreeMap<Position, (Position, Position)>, Position) {
    let mut cx = 0;
    let mut map = BTreeMap::new();
    let mut starting_pos = Position { x: 0, y: 0 };
    let _: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let cy = y as i32;
            let _: Vec<_> = line
                .chars()
                .map(|c| {
                    let from = Position { x: cx, y: y as i32 };
                    let mut to: (Position, Position) =
                        (Position { x: -10, y: -10 }, Position { x: -10, y: -10 });
                    match c {
                        '|' => to = (Position { x: cx, y: cy - 1 }, Position { x: cx, y: cy + 1 }),
                        '-' => to = (Position { x: cx - 1, y: cy }, Position { x: cx + 1, y: cy }),
                        'L' => to = (Position { x: cx, y: cy - 1 }, Position { x: cx + 1, y: cy }),
                        'J' => to = (Position { x: cx, y: cy - 1 }, Position { x: cx - 1, y: cy }),
                        '7' => to = (Position { x: cx, y: cy + 1 }, Position { x: cx - 1, y: cy }),
                        'F' => to = (Position { x: cx, y: cy + 1 }, Position { x: cx + 1, y: cy }),
                        '.' => to = (Position { x: cx, y: cy }, Position { x: cx, y: cy }),
                        'S' => {
                            to = (Position { x: cx, y: cy }, Position { x: cx, y: cy });
                            starting_pos = from;
                        }
                        _ => {}
                    };
                    map.insert(from, to);
                    cx += 1;
                })
                .collect();
            cx = 0;
        })
        .collect();
    (map, starting_pos)
}

fn part1(input: &str) -> String {
    let (map, starting_pos) = parse_input(input);
    let mut visited: Vec<Position> = vec![];

    for dir in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let mut current_pos = Position {
            x: starting_pos.x + dir.0,
            y: starting_pos.y + dir.1,
        };
        let mut from = starting_pos;
        visited = vec![starting_pos, current_pos];
        while current_pos != starting_pos {
            match map.get(&current_pos) {
                Some(next_pos) => {
                    if next_pos.0 == from {
                        from = current_pos;
                        current_pos = next_pos.1;
                    } else if next_pos.1 == from {
                        from = current_pos;
                        current_pos = next_pos.0;
                    } else {
                        break;
                    }
                }
                None => break,
            }
            visited.push(current_pos);
        }
        if visited.first() == visited.last() {
            break;
        }
    }
    (visited.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, "4".to_string());
    }
    #[test]
    fn test2() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, "8".to_string());
    }
}
