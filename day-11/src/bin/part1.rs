fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let lines: Vec<&str> = input.lines().collect();
    let (max_x, max_y) = (
        lines.len() - 1,
        lines[0].chars().collect::<Vec<char>>().len() - 1,
    );
    let gals: Vec<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(|(x, _c)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<(i32, i32)>>();

    let empty_rows: Vec<i32> = (0..=max_y as i32)
        .filter(|y| {
            !gals
                .iter()
                .map(|(_x, y)| *y)
                .collect::<Vec<i32>>()
                .contains(&(*y as i32))
        })
        .collect();

    let empty_cols: Vec<i32> = (0..=max_x as i32)
        .filter(|x| {
            !gals
                .iter()
                .map(|(x, _y)| *x)
                .collect::<Vec<i32>>()
                .contains(&(*x as i32))
        })
        .collect();

    let expanded_gals: Vec<(i32, i32)> = gals
        .iter()
        .map(|(x, y)| {
            let nb_empty_cols_smaller = empty_cols.iter().filter(|ec| *ec < x).count() as i32;
            let nb_empty_rows_smaller = empty_rows.iter().filter(|er| *er < y).count() as i32;
            (x + nb_empty_cols_smaller, y + nb_empty_rows_smaller)
        })
        .collect();
    expanded_gals
}

fn part1(input: &str) -> String {
    let gals = parse_input(input);
    let mut sum = 0;
    for i in 0..gals.len() {
        for j in i..gals.len() {
            if i == j {
                continue;
            }
            sum += (gals[i].0 - gals[j].0).abs() + (gals[i].1 - gals[j].1).abs();
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374".to_string());
    }
}
