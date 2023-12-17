use std::{collections::BTreeMap, ops::Add, vec};

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

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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

fn check_if_adjacent_to_existing(
    pos: Position,
    from: Position,
    visited: &Vec<Position>,
) -> Option<Vec<Position>> {
    let mut cycle: Option<Vec<Position>> = None;
    for dir in vec![
        Position { x: 0, y: 1 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: -1 },
        Position { x: -1, y: 0 },
    ] {
        let to_check = pos + dir;
        if to_check == from {
            continue;
        }
        match visited.iter().enumerate().find(|(_, v)| **v == to_check) {
            Some(elem) => {
                cycle = Some(visited.get(elem.0..visited.len()).unwrap().to_vec());
                break;
            }
            None => {}
        }
    }
    cycle
}

fn display(cycles: Vec<Vec<Position>>) {
    dbg!(cycles.len());
    for y in 0..10 {
        for x in 0..20 {
            let mut ch: Option<char> = None;
            for (i, cycle) in cycles.iter().enumerate() {
                if cycle.contains(&Position { x, y }) {
                    // ch = Some('c');
                    ch = Some(char::from_digit(i as u32, cycles.len() as u32).unwrap());
                }
            }
            match ch {
                Some(c) => print!("{c}"),
                None => print!("."),
            }
        }
        println!("");
    }
}

fn nb_enclosed(cycle: Vec<Position>) -> usize {
    0
}

fn part1(input: &str) -> String {
    let (map, starting_pos) = parse_input(input);
    let mut visited: Vec<Position> = vec![];
    let mut cycles: Vec<Vec<Position>> = vec![];

    for dir in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let mut current_pos = Position {
            x: starting_pos.x + dir.0,
            y: starting_pos.y + dir.1,
        };
        let mut from = starting_pos;
        visited = vec![starting_pos, current_pos];
        let mut last_i = 0;
        cycles.clear();
        while current_pos != starting_pos {
            let mut cycle: Option<Vec<Position>> = None;
            match map.get(&current_pos) {
                Some(next_pos) => {
                    if next_pos.0 == from {
                        cycle = check_if_adjacent_to_existing(
                            current_pos,
                            from,
                            &visited.get(last_i..).unwrap().to_vec(),
                        );
                        from = current_pos;
                        current_pos = next_pos.1;
                    } else if next_pos.1 == from {
                        cycle = check_if_adjacent_to_existing(
                            current_pos,
                            from,
                            &visited.get(last_i..).unwrap().to_vec(),
                        );
                        from = current_pos;
                        current_pos = next_pos.0;
                    } else {
                        break;
                    }
                }
                None => break,
            }
            visited.push(current_pos);
            match cycle {
                Some(c) => {
                    if c.len() > 4 {
                        cycles.push(c);
                        last_i = visited.len()
                    }
                }
                None => {}
            }
        }
        if visited.first() == visited.last() {
            break;
        }
    }
    display(cycles);
    (visited.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test() {
    //         let result = part1(
    //             "...........
    // .S-------7.
    // .|F-----7|.
    // .||.....||.
    // .||.....||.
    // .|L-7.F-J|.
    // .|..|.|..|.
    // .L--J.L--J.
    // ...........",
    //         );
    //         assert_eq!(result, "4".to_string());
    //     }
    #[test]
    fn test2() {
        let result = part1(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, "8".to_string());
    }
}
