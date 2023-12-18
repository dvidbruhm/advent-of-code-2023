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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rock {
    x: usize,
    y: usize,
    t: RockType,
}

fn move_rocks(rocks: &mut Vec<Rock>, dir: Direction, max_x: usize, max_y: usize) -> Vec<Rock> {
    match dir {
        Direction::Up => {
            rocks.sort_by_key(|rock| rock.y);
            rocks.sort_by_key(|rock| rock.x);
        }
        Direction::Down => {
            rocks.sort_by_key(|rock| rock.y);
            rocks.reverse();
            rocks.sort_by_key(|rock| rock.x);
        }
        Direction::Left => {
            rocks.sort_by_key(|rock| rock.x);
            rocks.sort_by_key(|rock| rock.y);
        }
        Direction::Right => {
            rocks.sort_by_key(|rock| rock.x);
            rocks.reverse();
            rocks.sort_by_key(|rock| rock.y);
        }
    }

    for i in 0..rocks.len() {
        let mut rock = rocks[i];
        let mut prev_rock: Option<Rock> = None;
        if i > 0 {
            if (rocks[i - 1].x == rock.x && vec![Direction::Up, Direction::Down].contains(&dir))
                || (rocks[i - 1].y == rock.y
                    && vec![Direction::Right, Direction::Left].contains(&dir))
            {
                prev_rock = Some(rocks[i - 1]);
                // display(rocks, max_x, max_y);
            }
        }
        match rock.t {
            RockType::Round => match prev_rock {
                Some(pr) => match dir {
                    Direction::Up => rock.y = pr.y + 1,
                    Direction::Down => rock.y = pr.y - 1,
                    Direction::Left => rock.x = pr.x + 1,
                    Direction::Right => rock.x = pr.x - 1,
                },
                None => match dir {
                    Direction::Up => rock.y = 0,
                    Direction::Down => rock.y = max_y - 1,
                    Direction::Left => rock.x = 0,
                    Direction::Right => rock.x = max_x - 1,
                },
            },
            RockType::Block => {}
        }
        rocks[i] = rock;
    }
    rocks.to_vec()
}

fn parse_input(input: &str) -> (Vec<Rock>, usize, usize) {
    let mut max_x = 0;
    let max_y = input.lines().count();

    let rocks = input
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

    (rocks, max_x, max_y)
}

fn display(rocks: &Vec<Rock>, max_x: usize, max_y: usize) {
    for y in 0..max_y {
        for x in 0..max_x {
            match rocks.iter().find(|rock| rock.x == x && rock.y == y) {
                Some(rock) => match rock.t {
                    RockType::Round => print!("O"),
                    RockType::Block => print!("#"),
                },
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn part1(input: &str) -> String {
    let (rocks, max_x, max_y) = parse_input(input);
    let cycle = vec![
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    let mut new_rocks: Vec<Rock> = rocks.clone();
    let mut prev_cycles: Vec<Vec<Rock>> = vec![];

    let mut i = 0;
    let mut current_cycle = 1;
    let mut last = 0;
    let mut rests: i32 = -1;
    let mut loop_len: i32 = -1;
    cycle
        .iter()
        .cycle()
        .take(4 * 1000000000)
        .try_for_each(|dir| {
            new_rocks = move_rocks(&mut new_rocks, *dir, max_x, max_y);

            if (i + 1) % 4 == 0 && rests == -1 {
                if prev_cycles.contains(&new_rocks) {
                    last = prev_cycles.iter().position(|pc| pc == &new_rocks).unwrap();
                    prev_cycles.push(new_rocks.clone());
                    loop_len = (prev_cycles.len() - 1 - last) as i32;
                    rests = (1000000000 - current_cycle) % (loop_len) * 4;
                }
                prev_cycles.push(new_rocks.clone());
                current_cycle += 1;
            }

            if rests > 0 {
                rests -= 1;
            }

            if rests == 0 {
                return None;
            }

            i += 1;
            return Some(());
        });

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
        assert_eq!(result, "64".to_string());
    }
}
