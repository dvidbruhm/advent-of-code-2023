fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockType {
    Round,
    Block,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    x: usize,
    y: usize,
    t: RockType,
}

fn move_rocks(rocks: &mut Vec<Rock>, dir: Direction, max_x: usize, max_y: usize) -> Vec<Rock> {
    for i in 0..rocks.len() {
        let mut rock = rocks[i];
        let mut prev_rock: Option<Rock> = None;
        if i > 0 {
            if rocks[i - 1].x == rock.x {
                prev_rock = Some(rocks[i - 1]);
            }
        }
        match rock.t {
            RockType::Round => match prev_rock {
                Some(pr) => {
                    rock.y = pr.y + 1;
                }
                None => {
                    rock.y = 0;
                }
            },
            RockType::Block => {}
        }
        rocks[i] = rock;
    }
    rocks.to_vec()
}

fn parse_input(input: &str) -> (Vec<Rock>, usize, usize) {
    let mut max_x = 0;
    let mut max_y = input.lines().count();

    let mut rocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            max_x = line.len();
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c != '.')
                .map(|(x, c)| match c {
                    'O' => Rock {
                        x,
                        y,
                        t: RockType::Round,
                    },
                    '#' => Rock {
                        x,
                        y,
                        t: RockType::Block,
                    },
                    _ => Rock {
                        x,
                        y,
                        t: RockType::Round,
                    },
                })
                .collect::<Vec<Rock>>()
        })
        .collect::<Vec<Rock>>();

    rocks.sort_by_key(|rock| rock.y);
    rocks.sort_by_key(|rock| rock.x);
    (rocks, max_x, max_y)
}

fn part1(input: &str) -> String {
    let (mut rocks, max_x, max_y) = parse_input(input);
    let new_rocks = move_rocks(&mut rocks, Direction::Up, max_x, max_y);
    let sum: usize = new_rocks
        .iter()
        .filter(|rock| rock.t == RockType::Round)
        .map(|rock| max_y - rock.y)
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, "136".to_string());
    }
}
