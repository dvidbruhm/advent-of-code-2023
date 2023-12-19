use itertools::Itertools;
use std::io::Write;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq)]
struct Beam {
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
    visited: Vec<(Tile, i32, i32)>,
    active: bool,
}

impl Beam {
    fn update_pos(&mut self, max_x: usize, max_y: usize, tile: Tile) {
        let to_add = (tile, self.dir_x, self.dir_y);
        if self.visited.contains(&to_add) {
            self.active = false;
        } else {
            self.visited.push(to_add);
        }
        self.x += self.dir_x;
        self.y += self.dir_y;
        if self.x >= max_x as i32 || self.y >= max_y as i32 || self.x < 0 || self.y < 0 {
            self.active = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    x: i32,
    y: i32,
    t: char,
    ener: bool,
}

fn get_tile(map: &mut Vec<Tile>, x: i32, y: i32) -> &mut Tile {
    map.iter_mut()
        .find(|tile| tile.x == x && tile.y == y)
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<Tile>, usize, usize) {
    let mut max_x = 0;
    let max_y = input.lines().count();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            max_x = line.len();
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    x: x as i32,
                    y: y as i32,
                    t: c,
                    ener: false,
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Tile>>();
    (map, max_x, max_y)
}

fn step_beams(map: &mut Vec<Tile>, beams: &mut Vec<Beam>, max_x: usize, max_y: usize) {
    let mut new_beams: Vec<Beam> = vec![];
    beams
        .iter_mut()
        .filter(|beam| beam.active)
        .for_each(|beam| {
            // map.iter_mut()
            //     .find(|tile| tile.x == beam.x && tile.y == beam.y)
            //     .unwrap()
            //     .ener = true;
            let current_tile = get_tile(map, beam.x, beam.y);
            current_tile.ener = true;
            match current_tile.t {
                '.' => {}
                '/' => {
                    if beam.dir_x == 0 {
                        beam.dir_x = -beam.dir_y;
                        beam.dir_y = 0;
                    } else {
                        beam.dir_y = -beam.dir_x;
                        beam.dir_x = 0;
                    }
                }
                '\\' => {
                    if beam.dir_x == 0 {
                        beam.dir_x = beam.dir_y;
                        beam.dir_y = 0;
                    } else {
                        beam.dir_y = beam.dir_x;
                        beam.dir_x = 0;
                    }
                }
                '-' => {
                    if beam.dir_x == 0 {
                        beam.dir_x = -1;
                        beam.dir_y = 0;
                        let mut new_beam = Beam {
                            x: beam.x,
                            y: beam.y,
                            dir_x: 1,
                            dir_y: 0,
                            visited: beam.visited.clone(),
                            active: true,
                        };
                        new_beam.update_pos(max_x, max_y, current_tile.clone());
                        new_beams.push(new_beam);
                    }
                }
                '|' => {
                    if beam.dir_y == 0 {
                        beam.dir_y = -1;
                        beam.dir_x = 0;
                        let mut new_beam = Beam {
                            x: beam.x,
                            y: beam.y,
                            dir_x: 0,
                            dir_y: 1,
                            visited: beam.visited.clone(),
                            active: true,
                        };
                        new_beam.update_pos(max_x, max_y, current_tile.clone());
                        new_beams.push(new_beam);
                    }
                }
                _ => {}
            }
            beam.update_pos(max_x, max_y, current_tile.clone());
        });
    beams.append(&mut new_beams);
}

fn process(
    start_x: i32,
    start_y: i32,
    dx: i32,
    dy: i32,
    map: &mut Vec<Tile>,
    max_x: usize,
    max_y: usize,
) -> usize {
    let mut beams = vec![Beam {
        x: start_x,
        y: start_y,
        dir_x: dx,
        dir_y: dy,
        visited: vec![],
        active: true,
    }];
    let mut count = 0;
    while beams.iter().any(|beam| beam.active) {
        step_beams(map, &mut beams, max_x, max_y);
        beams = beams
            .into_iter()
            .unique_by(|beam| (beam.x, beam.y, beam.dir_x, beam.dir_y))
            .collect::<Vec<Beam>>();
        count += 1;
        std::io::stdout().flush().unwrap();
        print!("{count}\r");
    }
    println!();
    map.iter().map(|tile| tile.ener as usize).sum::<usize>()
}

fn part1(input: &str) -> String {
    let (mut map, max_x, max_y) = parse_input(input);
    let mut sums: Vec<usize> = vec![];

    for i in 0..max_x {
        let (start_x, start_y) = (i as i32, 0);
        let sum = process(start_x, start_y, 0, 1, &mut map, max_x, max_y);
        sums.push(sum);
        dbg!((start_x, start_y, sum));
        map.iter_mut().for_each(|tile| tile.ener = false);
        let (start_x, start_y) = (i as i32, max_y as i32 - 1);
        let sum = process(start_x, start_y, 0, -1, &mut map, max_x, max_y);
        sums.push(sum);
        dbg!((start_x, start_y, sum));
        map.iter_mut().for_each(|tile| tile.ener = false);
        let (start_x, start_y) = (0, i as i32);
        let sum = process(start_x, start_y, 1, 0, &mut map, max_x, max_y);
        sums.push(sum);
        dbg!((start_x, start_y, sum));
        map.iter_mut().for_each(|tile| tile.ener = false);
        let (start_x, start_y) = (max_x as i32 - 1, i as i32);
        let sum = process(start_x, start_y, -1, 0, &mut map, max_x, max_y);
        sums.push(sum);
        dbg!((start_x, start_y, sum));
        map.iter_mut().for_each(|tile| tile.ener = false);
    }
    sums.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, "51".to_string());
    }
}
