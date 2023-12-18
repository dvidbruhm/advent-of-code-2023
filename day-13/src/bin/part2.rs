fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<Vec<Vec<i32>>> {
    let parsed: Vec<_> = input
        .split("\n\n")
        .map(|g| {
            let v: Vec<_> = g
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => 1,
                            _ => 2,
                        })
                        .collect::<Vec<i32>>()
                })
                .collect();
            v
        })
        .collect();
    parsed
}

fn get_row(grid: &Vec<Vec<i32>>, row: usize) -> &Vec<i32> {
    &grid[row as usize]
}

fn get_col(grid: &Vec<Vec<i32>>, col: usize) -> Vec<i32> {
    grid.iter().map(|row| row[col as usize]).collect()
}

fn find_reflection(grid: &Vec<Vec<i32>>, vertical: bool, orig_answer: Option<usize>) -> usize {
    let mut answer = 0;
    let max_x = grid[0].len();
    let max_y = grid.len();
    let max_axis = if vertical { max_x } else { max_y };
    for col_nb in 1..max_axis {
        let start_left = col_nb - 1;
        let start_right = col_nb;
        for offset in 0..col_nb.min(max_axis - col_nb) {
            let (left, right) = (start_left - offset, start_right + offset);
            let are_equal = if vertical {
                get_col(&grid, left) == get_col(&grid, right)
            } else {
                get_row(&grid, left) == get_row(&grid, right)
            };
            match are_equal {
                true => {}
                false => break,
            }
            if left == 0 || right == max_axis - 1 {
                answer = if vertical { col_nb } else { col_nb * 100 };
                match orig_answer {
                    Some(orig_a) => {
                        if answer == orig_a {
                            answer = 0;
                        }
                    }
                    None => {}
                }

                break;
            }
        }
        if answer != 0 {
            break;
        }
    }
    answer
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);
    let sum: usize = parsed
        .iter()
        .map(|grid| {
            let len_x = grid[0].len();
            let len_y = grid.len();
            let mut orig_answer = find_reflection(grid, true, None);
            if orig_answer == 0 {
                orig_answer = find_reflection(grid, false, None);
            }
            for i in 0..len_x * len_y {
                let mut new_grid = grid.clone();
                let (x, y) = (i % len_x, i / len_x);
                new_grid[y][x] = 1 - new_grid[y][x];
                let mut answer = find_reflection(&new_grid, true, Some(orig_answer));
                if answer == orig_answer {
                    answer = 0;
                }
                if answer == 0 {
                    answer = find_reflection(&new_grid, false, Some(orig_answer));
                }
                if answer != 0 && orig_answer != answer {
                    return answer;
                }
            }
            orig_answer
        })
        // .inspect(|ans| {
        //     dbg!(ans);
        // })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, "400".to_string());
    }
}
